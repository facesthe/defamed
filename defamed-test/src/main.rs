use submodule::exported_function;

mod submodule;

fn main() {
    unimplemented!("There is no main program. Run tests instead");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingest_custom_syntax() {
        let tokens = quote::quote! {
            fn aaa(pos: bool = 0) -> bool {
                false
            }
        };

        println!("{:#?}", tokens);

        let x: i32 = Default::default();

        // assert_eq!(aaa(true), false);
    }
}

// struct Item {}

// impl Item {}

// macro_rules! with_receiver {
//     ($slf: expr, param_a = $param_a_val: expr) => {};
// }

/// Hwllo everynyan
#[defamed::defamed]
pub fn asd(first_item: i32, #[def] second_item: i32) -> i32 {
    0
}

#[defamed::defamed]
fn qwe(one: i32, two: usize, three: bool) {}

pub fn something() {
    asd!(0, second_item = 20);
    asd!(first_item = 1);
    asd(0, 0);

    exported_function!();
    // can then be used like:
    // let x = some_function!(true, 10);
    // let x = some_function!(sign = false, value = 100);
    // let x = some_function!(value = 20, sign = false, div = 2);
    // let x = some_function!(true, 10, add = -10);
}

mod inner {
    /// ASdalksdasl k
    #[defamed::defamed]
    pub fn some_function(
        sign: bool,
        value: i32,
        // use #[default] for types that implement Default::default()
        #[def] add: i32,
        #[def(0)] div: i32,
    ) -> i32 {
        (if sign { value + add } else { 0 - value + add }) / div
    }
}

mod log {
    mod macros {
        macro_rules! info {
            ($v:expr) => {};
        }

        pub(crate) use info;
    }

    pub use macros::*;

    pub fn info(v: &str) {}
}

fn used() {
    log::info!("");
}

// #[test]
fn test() {
    let x = defamed_test_lib::ExportedStruct {};

    x.exported_method();

    // defamed_test_lib::
    // defamed_test_lib::exported_struct_macros::exported_method!(x);
}

// asd!(first_item  = 0);
