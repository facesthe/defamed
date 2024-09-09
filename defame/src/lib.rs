//!

mod params;

pub(crate) use proc_macro as pm;
pub(crate) use proc_macro2 as pm2;

#[proc_macro_attribute]
pub fn defame(attrs: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {





    input
}
