//! Function macro generators

use proc_macro2 as pm2;

/// Generate a macro with all permutations of positional, named and default parameters.
pub fn generate_func(
    func_ident: syn::Ident,
    params: crate::block_logic::ProcOutput,
) -> pm2::TokenStream {
    todo!()
}

fn some_func(
    // positional and named are the same
    pos_a: bool,
    pos_b: bool,

    // will have the #[default] attribute
    opt_a: bool,
) {
}

macro_rules! some_func (
    ($pos_a_val: expr, $pos_b_val: expr) => {
        some_func($pos_a_val, $pos_b_val, false);
    };
    (pos_a = $pos_a_val: expr, pos_b = $pos_b_val: expr) => {
        some_func($pos_a_val, $pos_b_val, false);
    };
    (pos_a = $pos_a_val: expr, pos_b = $pos_b_val: expr, opt_a = $opt_a_val: expr) => {
        some_func($pos_a_val, $pos_b_val, false);
    };
);

fn test() {
    some_func!(pos_a = !false, pos_b = false, opt_a = true);
    some_func!(!false, false);
    // some_func!(pos_a = true, pos_b = false, opt_a = true);
}
