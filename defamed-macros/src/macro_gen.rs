//! Function macro generators

use proc_macro2::{self as pm2, Span};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated,
    token::{Comma, Semi},
    Visibility,
};

use crate::{params::PermutedParam, traits::ToMacroPattern};

/// Generate a macro with all permutations of positional, named and default parameters.
/// The macro inherits all doc comments from the original function.
///
/// This macro generates code that calls the actual function,
/// while reorderng and substituting parameters as needed.
pub fn generate_func_macro(
    vis: Visibility,
    doc_attrs: Vec<syn::Attribute>,
    func_path: Option<pm2::Group>,
    func_ident: syn::Ident,
    params: Vec<Vec<PermutedParam>>,
) -> pm2::TokenStream {
    // first pattern contains the correct order of parameteres to call
    let first_ref = params
        .first()
        .cloned()
        .expect("at least one match pattern expected");

    let func_path = func_path
        .and_then(|g| Some(g.to_token_stream()))
        .unwrap_or(quote! {});

    let mut macro_matches: Punctuated<pm2::TokenStream, Semi> = params
        .into_iter()
        .map(|p| {
            let macro_signature = create_macro_signature(&p);
            let func_signature = create_func_call_signature(first_ref.as_slice(), &p);

            quote! {
                (#macro_signature) => {
                    // let x = module_path!();
                    // let y = x.split("::").collect::<Vec<_>>();
                    // println !("{:?}", y);
                    // stringify!(module_path!());
                    // println!("{}", $module_path!())
                    #func_ident(#func_signature)
                }
            }
        })
        .collect();

    macro_matches.push(quote! {
        ($other: tt) => {
            compile_error!("Invalid parameters")
        }
    });

    let macro_mod = syn::Ident::new(
        &format!("{}_macros", func_ident.to_token_stream().to_string()),
        Span::call_site(),
    );

    let macro_def_attr = match &vis {
        Visibility::Public(_) => quote! {#[macro_export]},
        Visibility::Restricted(_) | Visibility::Inherited => quote! {},
    };

    let func_dunder_ident = syn::Ident::new(
        &format!("__{}__", func_ident.to_token_stream().to_string()),
        Span::call_site(),
    );

    let doc_attr_tokens: pm2::TokenStream =
        doc_attrs.into_iter().map(|a| a.to_token_stream()).collect();

    quote! {
        // #vis mod #macro_mod {

            #[doc(hidden)]
            #macro_def_attr
            macro_rules! #func_dunder_ident (
                #macro_matches
            );

            #[doc(inline)]
            #doc_attr_tokens
            #vis use #func_dunder_ident as #func_ident;
        // }

        // #vis use #macro_mod::*;
        // #vis use #func_ident!;
    }
}

/// Create the macro pattern signature for a given vector of parameters.
fn create_macro_signature(params: &[PermutedParam]) -> pm2::TokenStream {
    let seq: Punctuated<pm2::TokenStream, Comma> =
        params.iter().filter_map(|p| p.to_macro_pattern()).collect();

    seq.to_token_stream()
}

fn create_func_call_signature(
    reference: &[PermutedParam],
    params: &[PermutedParam],
) -> pm2::TokenStream {
    let seq: Punctuated<pm2::TokenStream, Comma> = reference
        .iter()
        .map(|r| {
            let p = params
                .iter()
                .find(|item| *item == r)
                .expect("parameter must exist");

            p.to_func_call_pattern()
        })
        .collect();

    seq.to_token_stream()
}
