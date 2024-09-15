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
    package_name: &str,
    func_path: Option<syn::Path>,
    func_ident: syn::Ident,
    params: Vec<Vec<PermutedParam>>,
) -> pm2::TokenStream {
    // first pattern contains the correct order of parameteres to call
    let first_ref = params
        .first()
        .cloned()
        .expect("at least one match pattern expected");

    let func_path_mid = func_path
        .clone()
        .and_then(|g| Some(quote! {#g ::}))
        .unwrap_or(quote! {});

    let package_ident = syn::Ident::new(&package_name.replace("-", "_"), Span::call_site());

    let macro_matches: Punctuated<pm2::TokenStream, Semi> = params
        .into_iter()
        .map(|p| {
            let macro_signature = create_macro_signature(&p);
            let func_signature = create_func_call_signature(first_ref.as_slice(), &p);

            match &vis {
                // pub macros require differentiation between internal and external calls
                Visibility::Public(_) => vec![
                    quote! {
                        (crate: #macro_signature) => {
                            crate :: #func_path_mid #func_ident(#func_signature)
                        }
                    },
                    quote! {
                        (#macro_signature) => {
                            #package_ident :: #func_path_mid #func_ident(#func_signature)
                        }
                    },
                ]
                .into_iter(),
                Visibility::Restricted(_) => vec![quote! {
                    (#macro_signature) => {
                        crate :: #func_path_mid #func_ident(#func_signature)
                    }
                }]
                .into_iter(),
                Visibility::Inherited => vec![quote! {
                    (#macro_signature) => {
                        #func_ident(#func_signature)
                    }
                }]
                .into_iter(),
            }
        })
        .flatten()
        .collect();

    let _macro_mod = syn::Ident::new(
        &format!("{}_macros", func_ident.to_token_stream().to_string()),
        Span::call_site(),
    );

    let macro_def_attr = match &vis {
        Visibility::Public(_) => quote! {#[macro_export]},
        Visibility::Restricted(_) | Visibility::Inherited => quote! {},
    };

    let func_dunder_ident = syn::Ident::new(
        &format!(
            "__{}{}__",
            match &func_path {
                Some(p) => format!("{}_", p.to_token_stream()),
                None => "".to_string(),
            },
            func_ident.to_token_stream().to_string()
        ),
        Span::call_site(),
    );

    // let full_func_path = match func_path {
    //     Some(p) => quote! {crate::#p::#func_ident},
    //     None => quote! {crate::#func_ident},
    // };

    quote! {
        // #vis mod #macro_mod {

            #[doc(hidden)]
            #[allow(unused_macros)]
            #macro_def_attr
            macro_rules! #func_dunder_ident (
                #macro_matches
            );

            #[doc(inline)]
            // #[allow(unused_macros)]
            #[doc = concat!("[`defamed`] wrapper for [`", stringify!(#func_ident), "()`]")]
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
