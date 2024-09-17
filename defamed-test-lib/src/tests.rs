#![cfg(test)]
use super::*;

#[test]
fn test_complex_function() {
    let _ = complex_function!(1, 2);

    assert_eq!(complex_function!(10, 5), 15);
    assert_eq!(complex_function!(10, 5, add = false), 5);
    assert_eq!(complex_function!(10, 20, divide_result_by = Some(2)), 15);

    // all arguments can be named
    assert_eq!(
        complex_function!(lhs = 20, rhs = 10, add = false, divide_result_by = Some(2)),
        5
    );
    // positional arguments can be named in any order, but must be provided before default arguments
    assert_eq!(
        complex_function!(rhs = 10, lhs = 20, divide_result_by = Some(2), add = false),
        5
    );
    assert_eq!(complex_function!(20, 10, false, Some(2)), 5);
}
