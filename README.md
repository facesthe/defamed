# defamed
Default, named and positional function arguments.

## Introduction
When writing code in Python, it is possible to do the following:

```python
# this function
def some_function(
    sign: bool,
    value: int,
    add: int = 0,
    div: int = 1
) -> int:
    if sign:
        return (value + add) / div
    else:
        return (0 - value + add) / div

# can be used like this:
assert some_function(True, 10) == 10
assert some_function(value = 20, sign = False, div = 2) == -10
assert some_function(True, 10, add = -10) == 0
```

This macro generates a function macro that replicates the features above.
```rust
// this function
#[defamed::defamed]
fn some_function(
    sign: bool,
    value: i32,
    // for types that implement std::Default
    #[default]
    add: i32,
    // any const expression can be a default
    #[default(1 + 2 - 1)]
    div: i32
) -> i32 {
    if sign {
        value + add
    } else {
        0 - value + add
    } / div
}

// can then be used like:
assert!(some_function!(true, 10) == 10);
assert!(some_function!(value = 20, sign = false, div = 2) == -10);
assert!(some_function!(true, 10, add = -10) == 0);
```
