#![cfg_attr(not(doctest), doc = include_str!("../../README.md"))]

mod block_logic;
mod macro_gen;
mod params;
mod traits;

use proc_macro as pm;
use quote::ToTokens;
use syn::spanned::Spanned;

/// Identifier for public macros defined in the root module
pub(crate) const ROOT_VISIBILITY_IDENT: &str = "crate";

/// "Helper" attribute for annotating function parameters
pub(crate) const DEFAULT_HELPER_ATTR: &str = "def";

/// Create a wrapper macro that accepts positional and arbitrarily ordered named arguments.
///
/// ## Example
/// ```
/// #[defamed::defamed]
/// fn complex_function(
///     base: i32,
///     other: i32,
///     // literals can be used as default values
///     #[def(true)] add: bool,
///     // if no default value is provided, the type must implement Default
///     #[def] divide_result_by: Option<i32>,
/// ) -> i32 {
///     let intermediate = if add { base + other } else { base - other };
///     match divide_result_by {
///         Some(div) => intermediate / div,
///         None => intermediate,
///     }
/// }
///
/// assert_eq!(complex_function!(10, 5), 15);
/// assert_eq!(complex_function!(10, 5, add = false), 5);
/// assert_eq!(complex_function!(10, 20, divide_result_by = Some(2)), 15);
///
/// // the original function is left unchanged, but
/// // the macro can be used as a drop-in replacement
/// assert_eq!(
///     complex_function(10, 20, true, Some(2)),
///     complex_function!(10, 20, true, Some(2)),
/// );
///
/// // all arguments can be named
/// assert_eq!(
///     complex_function!(
///         base = 20,
///         other = 10,
///         add = false,
///         divide_result_by = Some(2)
///     ), 5
/// );
/// // positional arguments can be named in any order, but must be provided before default arguments
/// assert_eq!(
///     complex_function!(
///         other = 10,
///         base = 20,
///         divide_result_by = Some(2),
///         add = false
///     ), 5
/// );
/// ```
#[proc_macro_attribute]
pub fn defamed(attrs: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let package_name =
        std::env::var("CARGO_PKG_NAME").expect("every crate must have a package name");

    let fn_path = match attrs.is_empty() {
        true => None,
        false => {
            // asd
            let ex: syn::Expr = syn::parse_macro_input!(attrs);

            if let syn::Expr::Path(syn::ExprPath { path, .. }) = &ex {
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
#[doc(hidden)]
#[proc_macro]
pub fn resolve_crate_path(_input: pm::TokenStream) -> pm::TokenStream {
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

    lit_str.to_token_stream().into()

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
