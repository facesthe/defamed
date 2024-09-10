//! Function macro generators

use proc_macro2 as pm2;
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated,
    token::{Comma, Semi},
};

use crate::{
    params::{self, PermutedParam},
    traits::ToMacroPattern,
};

/// Generate a macro with all permutations of positional, named and default parameters.
///
/// This macro generates code that calls the actual function,
/// while reorderng and substituting parameters as needed.
pub fn generate_func_macro(
    func_ident: syn::Ident,
    params: Vec<Vec<PermutedParam>>,
) -> pm2::TokenStream {
    // first pattern contains the correct order of parameteres to call
    let first_ref = params.get(0).expect("at least one match pattern expected");

    let macro_matches: Punctuated<pm2::TokenStream, Semi> = params
        .into_iter()
        .map(|p| {
            let macro_signature = create_macro_signature(p);

            quote! {
                (#macro_signature) => {
                    // to be replaced with actual function call
                    // #func_ident(#(#p),*)
                }
            }

            //   asd
        })
        .collect();

    quote! {
        macro_rules! #func_ident {
            #macro_matches
        }
    }
}

/// Create the macro pattern signature for a given vector of parameters.
fn create_macro_signature(params: Vec<PermutedParam>) -> pm2::TokenStream {
    let seq: Punctuated<pm2::TokenStream, Comma> =
        params.iter().filter_map(|p|{ p.to_macro_pattern()
        }).collect();

    seq.to_token_stream()
}

fn some_func(
    // positional and named are the same
    pos_a: bool,
    pos_b: bool,

    // will have the #[default] attribute
    opt_a: bool,
) {
}

macro_rules! some_func (
    ($pos_a_val: expr, $pos_b_val: expr) => {
        some_func($pos_a_val, $pos_b_val, false);
    };
    (pos_a = $pos_a_val: expr, pos_b = $pos_b_val: expr) => {
        some_func($pos_a_val, $pos_b_val, false);
    };
    (pos_a = $pos_a_val: expr, pos_b = $pos_b_val: expr, opt_a = $opt_a_val: expr) => {
        some_func($pos_a_val, $pos_b_val, false);
    };
);

fn test() {
    some_func!(!false, false);
    some_func!(pos_a = !false, pos_b = false, opt_a = true);
    // some_func!(pos_a = true, pos_b = false, opt_a = true);
}
