#![allow(unused)]
//!

mod block_logic;
mod params;
mod macro_gen;
mod traits;

pub(crate) use proc_macro as pm;
pub(crate) use proc_macro2 as pm2;
use quote::ToTokens;

pub(crate) const DEFAULT_ATTR: &str = "default";

#[proc_macro_attribute]
pub fn defame(_: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let x = syn::parse::<syn::ItemFn>(input.clone());

    let res = match (syn::parse::<syn::ItemFn>(input.clone())) {
        Ok(input) => block_logic::item_fn(input),
        Err(e) => e.to_compile_error().into(),
    };

    res.into()
}
