//!

mod block_logic;
mod macro_gen;
mod params;
mod traits;

use proc_macro as pm;

pub(crate) const DEFAULT_ATTR: &str = "default";

#[proc_macro_attribute]
pub fn defamed(_: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    // let x = syn::parse::<syn::ItemFn>(input.clone());

    let res = match syn::parse::<syn::ItemFn>(input.clone()) {
        Ok(input) => block_logic::item_fn(input),
        Err(e) => e.to_compile_error().into(),
    };

    res.into()
}
