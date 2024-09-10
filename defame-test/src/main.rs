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
#[defame::defame]
fn asd(first_item: i32, #[default] second_item: i32) -> i32 {
    0
}


fn test() {
    asd!(0, second_item = 0);
}

// asd!(first_item  = 0);
