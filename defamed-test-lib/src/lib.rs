//! Test lib for defamed
//!

pub struct ExportedStruct {}

impl ExportedStruct {
    pub fn exported_method(&self) {}
}

pub mod exported_struct_macros {
    // #[doc(hidden)]
    #[macro_export]
    macro_rules! __exported_method {
        ($self: expr) => {};
    }

    // #[doc(inline)]
    pub use __exported_method as exported_struct__exported_method;
    pub use __exported_method as exported_method;
}

#[allow(unused_imports)]
pub use exported_struct_macros::*;

fn ass() {}

///a asdasd
#[named::named(defaults(b = false))]
fn named_fn(a: bool, b: bool) -> bool {
    false
}

fn use_named_fn() {
    named_fn!(a = false);
}
