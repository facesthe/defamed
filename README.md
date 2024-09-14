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

This macro generates a function macro that replicates the features above:
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

## Notes 4 me
- Determine macro invocation semantics
    - no DSL (function macros only)
    - attr macro w/ pseudo helper-attrs
- Determine param permutations a-la Python
- Exporting macro in module (! @ crate root) based on visibility:
    - main issue: https://github.com/rust-lang/rust/issues/59368
    - fix: https://github.com/rust-lang/rust/pull/108241
    - rust-analyzer hinting has issues
- Problem when invoking macro from extern module/crate
    - similar crates do not export macro with function (named, etc..)
    - inner function requires fully qualified path
    - attempt 1: module_path!() macro
        - macro needs to expand after insertion in attributed code
        - parse &str from compiler builtin macro
        - macros evaluate lazily -> outer macro receives ItemMacro tokens
        - possible, but requires nightly
    - other attempts:
        - caller_modpath: https://docs.rs/caller_modpath/latest/caller_modpath/
            - also requires nightly

        - eager: https://docs.rs/eager/latest/eager/macro.eager.html
            - does not expand builtin macro
    - crate name eval can be done at compile time using proc-macros
        - evaluate "CARGO_PKG_NAME" env var inside macro

- Current (temp) solution: define crate path path as a parameter in attribute

- New (iffy) solution: multi stage macros
    - this solution requires that this library is also included by the user in their crate (double import)
    - first proc-macro generates actual function macro with all permutations
    and exports function macro under module scope
    - when called, function macro resolves to another proc-macro to eval crate root path (crate:: or otherwise).
    this proc-macro is provided by this crate, hence the need to double import
    - final function substituted in code

- New (less iffy solution): more macro permutations!
    - every macro permutation now has 2 variants: a crate-wide invocation and a public invocation.
    - any macro not called in the same scope as it was defined will need the fully qualified path of it's invoked inner function
    - a `crate:` prefix indicates that the macro substitutes code for invocation inside it's own crate
    - no prefix indicates that code should be substituted for users of that crate
