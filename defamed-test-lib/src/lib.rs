//! Test lib for defamed
//!

mod tests;

/// This function is public, so it can be used by other crates as well as internally.
///
/// ```
/// let add_default = defamed_test_lib::some_root_function!("base");
/// let add_known = defamed_test_lib::some_root_function!("base", concat = Some(" concat"));
///
/// assert_eq!(add_default.as_str(), "base");
/// assert_eq!(add_known.as_str(), "base concat");
/// ```
#[defamed::defamed(crate)]
pub fn some_root_function(base: &str, #[def] concat: Option<&str>) -> String {
    let _ = complex_function!(1, 2);

    match concat {
        Some(c) => base.to_owned() + c,
        None => base.to_owned(),
    }
}

pub mod inner {
    /// Mask the base value with a mask and shift the result right by `r_shift` bits.
    /// Returns `true` if the LSB of the result is set, `false` otherwise.
    #[defamed::defamed(inner)]
    pub fn nested_inner_function(base: u8, mask: u8, #[def] r_shift: u8) -> bool {
        let inter = base & mask;
        let shifted = inter >> r_shift;

        shifted & 1 != 0
    }
}

#[defamed::defamed]
fn complex_function(
    base: i32,
    other: i32,
    // literals can be used as default values
    #[def(true)] add: bool,
    // if no default value is provided, the type must implement Default
    #[def] divide_result_by: Option<i32>,
) -> i32 {
    let intermediate = if add { base + other } else { base - other };

    match divide_result_by {
        Some(div) => intermediate / div,
        None => intermediate,
    }
}
