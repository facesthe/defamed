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

#[test]
fn test_default_struct() {
    let a = DefaultStruct! {
        index: 0,
        ..
    };

    let b = DefaultStruct! {
        index: 1,
        inner: &[1, 2, 3],
        ..
    };

    let c = DefaultStruct! {
        index: 1,
        offset: 1,
        inner: &[1, 2, 3]
    };

    assert_eq!(a.value_at(), None);
    assert_eq!(b.value_at(), Some(2));
    assert_eq!(c.value_at(), Some(3));
}

#[test]
fn test_default_tuple_struct() {
    let a = DefaultTupleStruct!(1);
    let b = DefaultTupleStruct!(1, 2);
    let c = DefaultTupleStruct!(2, 4, 'f');

    assert_eq!(a, DefaultTupleStruct(1, 0, 'a'));
    assert_eq!(b, DefaultTupleStruct(1, 2, 'a'));
    assert_eq!(c, DefaultTupleStruct(2, 4, 'f'));
}
