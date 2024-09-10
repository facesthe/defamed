#![allow(unused)]
//!

mod block_logic;
mod params;

pub(crate) use proc_macro as pm;
pub(crate) use proc_macro2 as pm2;
use quote::ToTokens;

pub(crate) const DEFAULT_ATTR: &str = "default";

#[proc_macro_attribute]
pub fn defame(attrs: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(input as syn::ItemFn);

    let params = match params::FunctionParams::from_punctuated(sig.inputs.clone()) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    let stripped_attrs = params.to_punctuated();
    let mut new_sig = sig.clone();
    new_sig.inputs = stripped_attrs;

    // placeholder
    syn::ItemFn {
        attrs,
        vis,
        sig: new_sig,
        block,
    }
    .to_token_stream()
    .into()
}
