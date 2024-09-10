# defamed
Default, named and positional function arguments.

## Introduction
When writing code in Python, it is possible to do the following:

```python
# this function
def some_function(
    sign: bool,
    value: int,
    default: int = 10
) -> int:
    if sign:
        return value + default
    else:
        return 0 - value + default

# can be used like this:
assert some_function(True, 10) == 20
assert some_function(value = 20, sign = False) == -10
assert some_function(True, 10, default = -10) == 0
```

This macro generates a function macro that replicates the features above.
```rust
// this function
#[defame::defame]
fn some_function(
    sign: bool,
    value: i32,
    // use #[default] for types that implement Default::default()
    #[default(10)]
    default: i32
) -> i32 {
    if sign {
        value + default
    } else {
        0 - value + default
    }
}

// can then be used like:
assert!(some_function!(true, 10) == 20);
assert!(some_function!(value = 20, sign = false) == -10);
assert!(some_function!(true, 10, default = -10) == 0);
```
