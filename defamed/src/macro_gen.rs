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
///
/// This macro generates code that calls the actual function,
/// while reorderng and substituting parameters as needed.
pub fn generate_func_macro(
    vis: Visibility,
    func_ident: syn::Ident,
    params: Vec<Vec<PermutedParam>>,
) -> pm2::TokenStream {
    // first pattern contains the correct order of parameteres to call
    let first_ref = params
        .first()
        .cloned()
        .expect("at least one match pattern expected");

    let macro_matches: Punctuated<pm2::TokenStream, Semi> = params
        .into_iter()
        .map(|p| {
            let macro_signature = create_macro_signature(&p);
            let func_signature = create_func_call_signature(first_ref.as_slice(), &p);

            quote! {
                (#macro_signature) => {
                    #func_ident(#func_signature)
                }
            }
        })
        .collect();

    let macro_mod = syn::Ident::new(
        &format!("{}_macros", func_ident.to_token_stream().to_string()),
        Span::call_site(),
    );

    quote! {
        mod #macro_mod {
            macro_rules! #func_ident (
                #macro_matches
            );

            pub(crate) use #func_ident;
        }

        #vis use #macro_mod::*;
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
