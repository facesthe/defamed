fn main() {
    println!("Hello, world!");
}

// #[defame::defame]
// fn aaa(pos: bool = 0) -> bool {
//     false
// }

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
fn asd(first_item: i32, #[default] second_item: i32) -> i32 {
    0
}

/// ASdalksdasl k
#[defamed::defamed]
fn some_function(
    sign: bool,
    value: i32,
    // use #[default] for types that implement Default::default()
    #[default] add: i32,
    #[default(1)] div: i32,
) -> i32 {
    (if sign { value + add } else { 0 - value + add }) / div
}

fn something() {
    asd!(0, second_item = 0);
    asd!(first_item = 1);

    // can then be used like:
    let x = some_function!(true, 10);
    let x = some_function!(sign = false, value = 100);
    let x = some_function!(value = 20, sign = false, div = 2);
    let x = some_function!(true, 10, add = -10);
}

#[test]
fn test() {}

// asd!(first_item  = 0);
