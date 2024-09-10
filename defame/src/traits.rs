use proc_macro2 as pm2;

/// Turn `Self` into a macro pattern.
///
/// ```no_run
/// macro_rules! some_macro (
///     ($pos_a_val: expr, $pos_b_val: expr) => {
///     //^^^^^^^^^^^^^^^ this is one pattern
///     // a macro match pattern consists of one or more patterns
///
///     }
/// )
/// ```
pub trait ToMacroPattern {
    fn to_macro_pattern(&self) -> Option<pm2::TokenStream>;
}
