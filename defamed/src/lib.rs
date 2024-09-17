#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

mod block_logic;
mod macro_gen;
mod params;
mod traits;

use proc_macro as pm;
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
    // let package_name =
    //     std::env::var("CARGO_PKG_NAME").expect("every crate must have a package name");

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
        Ok(input) => block_logic::item_fn(input, fn_path),
        Err(e) => e.to_compile_error().into(),
    };

    res.into()
}
