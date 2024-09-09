//! Function param stuff

/// Default function parameter
#[derive(Clone)]
pub struct FunctionParam {
    /// Param name
    name: String,
    ty: syn::Type,
    /// A const that can be used as a default value
    default_value: Option<syn::Expr>,
}
