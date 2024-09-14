#[defamed::defamed]
pub fn exported_function() {}

pub struct SomeStruct {}

macro_rules! some_macro {
    () => {};
}

impl SomeStruct {
    pub fn some_method(&self) {}
}

mod some_struct_macros {

    #[macro_export]
    #[doc(hidden)]
    macro_rules! __some_method {
        ($self: expr) => {};
    }

    #[doc(inline)]
    pub use __some_method as some_method;
}

// some_method!(SomeStruct {});
// some_struct_macros

// defamed_test_lib::macros::fiz!();
// defamed_test_lib::macros::fiz!();

fn submodule_fn() {
    // defamed_test_lib::defame_macros::some_defamed_function!(1, None);
}
// defamed_test_lib::defame_macros::some_defamed_function!(1, 2);
// defamed_test_lib::defame_macros::some_defamed_function!(1);
// defamed_test_lib::named_macros::some_named_function!(9);
