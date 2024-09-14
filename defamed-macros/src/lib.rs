//!

mod block_logic;
mod macro_gen;
mod params;
mod traits;

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, ExprGroup};

/// Identifier for public macros defined in the root module
pub(crate) const ROOT_VISIBILITY_IDENT: &str = "root";

/// "Helper" attribute for annotating function parameters
pub(crate) const DEFAULT_HELPER_ATTR: &str = "def";

#[proc_macro_attribute]
pub fn defamed(attrs: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let package_name =
        std::env::var("CARGO_PKG_NAME").expect("every crate must have a package name");

    let fn_path = match attrs.is_empty() {
        true => None,
        false => {
            // asd
            let ex: syn::Expr = syn::parse_macro_input!(attrs);

            if let syn::Expr::Path(syn::ExprPath { attrs, qself, path }) = &ex {
                // if let syn::Expr::Path(p) = expr.as_ref() {
                Some(path.clone())

                // } else {
            } else {
                return syn::Error::new(ex.span(), "Expected path expression")
                    .to_compile_error()
                    .into();
            }
        }
    };

    let res = match syn::parse::<syn::ItemFn>(input.clone()) {
        Ok(input) => block_logic::item_fn(input, &package_name, fn_path),
        Err(e) => e.to_compile_error().into(),
    };

    res.into()
}

/// Checks the current crate's package name.
/// This attribute macro should be placed above a function call.
///
/// The macro checks the current package name against the provided package name.
/// The provided package name must be a string literal.
/// If it matches, `crate::` is prepended to the function call.
/// If it does not match, `PACKAGE_NAME::` is prepended instead.
#[proc_macro]
pub fn resolve_crate_path(input: pm::TokenStream) -> pm::TokenStream {
    // let syn::ExprCall {
    //     attrs,
    //     func,
    //     paren_token,
    //     args,
    // } = parse_macro_input!(input);

    // let source_package = attrs.first().expect("expected one attribute");

    todo!()
}

#[doc(hidden)]
#[proc_macro]
pub fn path_from_const_lit(input: pm::TokenStream) -> pm::TokenStream {
    let macro_expr = syn::parse_macro_input!(input as syn::Expr);

    let expanded = quote::quote! {
        #macro_expr;
    };
    // return expanded.into();

    let lit_str = syn::parse2::<syn::ItemMacro>(expanded).unwrap();

    return lit_str.to_token_stream().into();

    // let inner_val = match const_lit.expr.as_ref() {
    //     syn::Expr::Lit(syn::ExprLit {
    //         // attrs,
    //         lit: syn::Lit::Str(s),
    //         ..
    //     }) => s.clone(),
    //     _ => panic!("expected literal expression"), // replace with compile error
    // };

    // todo!();

    // let str_val = lit_str.value();

    // let path_segments = str_val.split("::").collect::<Vec<_>>();

    // if !path_segments
    //     .iter()
    //     .all(|s| s.chars().all(|c| c.is_ascii()))
    // {
    //     panic!("non-ascii characters in path");
    // }

    // let idents: Punctuated<syn::Ident, syn::Token![::]> = path_segments
    //     .into_iter()
    //     .map(|p| syn::Ident::new(p, Span::call_site()))
    //     .collect();

    // idents.to_token_stream().into()
}
