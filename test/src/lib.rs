//! Test lib for defamed
//!

/// This function is public, so it can be used by other crates as well as internally.
///
/// ```
/// let add_default = defamed_test_lib::some_root_function!("base");
/// let add_known = defamed_test_lib::some_root_function!("base", concat = Some(" concat"));
///
/// assert_eq!(add_default.as_str(), "base");
/// assert_eq!(add_known.as_str(), "base concat");
/// ```
#[defamed::defamed(root)]
pub fn some_root_function(base: &str, #[def] concat: Option<&str>) -> String {
    match concat {
        Some(c) => base.to_owned() + c,
        None => base.to_owned(),
    }
}

mod conditional {}

pub mod defame_macros {

    /// ASPDMASPDLSMPPSLMPSLMPS
    ///
    /// This is a document comment
    #[defamed::defamed(defame_macros)]
    fn some_private_function(input_a: u8, input_b: Option<usize>) -> bool {
        false
    }

    /// This is a public function
    /// [crate::defame_macros]
    #[defamed::defamed(defame_macros)]
    pub fn some_public_function(input_a: u8, #[def] input_b: Option<usize>) -> bool {
        false
    }

    // #[doc(hidden)]
    // #[macro_export]
    // macro_rules! __some_defamed_function__(
    //         (input_a = $input_a_val : expr,
    //     input_b = $input_b_val : expr) =>
    //     { $crate :: some_defamed_function($input_a_val, $input_b_val) };

    //     (input_b = $input_b_val : expr, input_a = $input_a_val : expr) =>
    //     { some_defamed_function($input_a_val, $input_b_val) };

    //     ($input_a_val : expr, input_b = $input_b_val : expr) =>
    //     { $crate :: some_defamed_function($input_a_val, $input_b_val) };

    //     ($input_a_val : expr, $input_b_val : expr) =>
    //     { use crate::defame_macros::some_defamed_function; some_defamed_function($input_a_val, $input_b_val) }

    // );
    // #[doc(inline)]
    // pub use __some_defamed_function__ as some_defamed_function;

    fn testing() {
        // plain func
        crate::defame_macros::some_private_function(1, None);

        // crate::defame_macros::some_private_function!(1);
        // crate::defame_macros::some_private_function!(input_a = 1);
        // crate::defame_macros::some_private_function!(input_a = 1);

        some_private_function!(crate: 1, None);
        // crate::defame_macros::some_private_function!(input_a = 1, input_b = None);
        // crate::defame_macros::some_private_function!(input_b = None, input_a = 1);

        some_private_function!(crate: 0, None);
        // self::some_private_function!(input_b = Some(0), input_a = 1);
    }
}

fn asd() {
    // use crate as defamed_test_lib;
    // use defame_macros::some_defamed_function;
    // defamed_test_lib::defame_macros::some_defamed_function(1, None);
    // defamed_test_lib::defame_macros::some_defamed_function!(1, None);

    // check if we are calling internally
    if env!("CARGO_PKG_NAME") == "defamed-test-lib" {
        // crate::defame_macros::some_defamed_function(1, None);
    } else {
        // use defamed_test_lib::defame_macros;
        todo!();
        // defamed_test_lib::defame_macros::some_defamed_function(1, None);
    }
}

macro_rules! strip {
    ($litstr: literal) => {};
    () => {};
}

macro_rules! substitute {
    () => {};
}

fn something() {
    // defamed::resolve_crate_path!(
    //     "defamed-test-lib",
    // defame_macros::some_defamed_function(1, None);
    // );

    strip!("$asd: expr");
    strip!("\"");
    // module_path!();
    // use eager::eager_internal;
    // use eager::eager_macro_rules_internal;

    // eager_macro_rules! { $eager_1
    //     macro_rules! module_path_eager {
    //         () => {
    //             module_path!();
    //         };
    //     }
    // }

    // module_path_eager!();

    // defamed::path_from_const_lit!(eager::eager!(module_path_eager!()));
    // defamed::path_from_lit_str!(stringify!(1 + 2));
}

// #[allow(non_snake_case)]
// mod exported_struct {
//     #[doc(hidden)]
//     #[macro_export]
//     macro_rules! exported_method {
//         ($slf: expr) => {};
//     }

//     #[doc(inline)]
//     pub use exported_method;
// }

// fn asd() {
//     let s = ExportedStruct {};
//     exported_struct::exported_method!(s);
// }

// pub mod exported_struct_macros {
//     #[doc(hidden)]
//     #[macro_export]
//     macro_rules! __exported_method {
//         ($self: expr) => {};
//     }

//     #[doc(hidden)]
//     #[allow(unused)]
//     pub use __exported_method as exported_method;
// }

// #[allow(unused_imports)]
// pub use exported_struct_macros::*;
// fn ass() {}

// #[macro_export]
// macro_rules! scoped {(
//     $(
//         $( #$attr:tt )*
//         $( pub ($($restricted:tt)+) )?
//         $( pub $(@$if_pub:tt)?      )?
//         //     ^^^^^^^^^^^^^^^
//         //     `$if_pub:empty` matcher when? 🥺👉👈
//         macro_rules! $macro:ident $rules:tt
//     )*
// ) => (::paste::item! {
//     $(
//         $( #$attr )*
//         $($($if_pub)? #[doc(hidden)] #[macro_export] )?
//         macro_rules! [< __ $macro >] $rules

//         $( #$attr )*
//         #[doc(hidden)]
//         #[allow(unused)]
//         $(pub ($($restricted)+))? $($($if_pub)? pub)?
//         use [< __ $macro >] as $macro;
//     )*
// })}

// // macros::foo! {}
// // macros::bar! {}
// // macros::baz! {}

// pub mod macros {
//     // scoped! {
//     //     pub macro_rules! foo {() => ()}
//     //     pub(crate) macro_rules! bar {() => ()}
//     //     macro_rules! baz {() => ()}
//     // }

//     #[doc(hidden)]
//     #[macro_export]
//     macro_rules! __fiz {
//         () => {};
//     }

//     #[doc(inline)]
//     pub use __fiz as fiz;
// }

// // #[allow()]
