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
    macro_rules! some_method {
        ($self: expr) => {};
    }

    pub(crate) use some_method;
}

use some_struct_macros::*;
some_method!(SomeStruct {});


