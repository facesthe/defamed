use proc_macro2 as pm2;

/// Turn `Self` into fragments of rust code.
pub trait ToMacroPattern {
    /// Create a macro match pattern.
    ///
    /// ```ignore
    /// macro_rules! some_macro (
    ///     ($pos_a_val: expr, $pos_b_val: expr) => {
    ///     //^^^^^^^^^^^^^^^ this is one pattern
    ///     // a macro match pattern consists of one or more comma-separated patterns.
    ///     // params that contain a default value and are not used should return None.
    ///
    ///     }
    /// )
    /// ```
    fn to_macro_pattern(&self) -> Option<pm2::TokenStream>;

    /// Create a function call pattern.
    ///
    /// ```ignore
    /// macro_rules! some_macro (
    ///     ($pos_a_val: expr, $pos_b_val: expr) => {
    ///         function_call($pos_a_val, $pos_b_val)
    ///     //                ^^^^^^^^^^ this is one pattern
    ///     // a function call pattern consists of one or more comma-separated patterns
    ///     }
    /// )
    /// ```
    fn to_func_call_pattern(&self) -> pm2::TokenStream;
}

/// Strip matching attributes from a type.
/// For function parameters, this is the `#[def]` attribute.
pub trait StripAttributes {
    /// The original type
    type Original;

    fn strip_attributes(&self) -> Self::Original;
}

/// Generate all permutations of a set of items.
pub trait GeneratePermutations {
    type Item;

    /// Generate all permutations of the items.
    fn generate_permutations(&self) -> Vec<Vec<Self::Item>>;
}
