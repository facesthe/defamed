# defamed
<div align="center">

#### Default, named and positional function parameters.

[![crate](https://img.shields.io/crates/v/defamed.svg)](https://crates.io/crates/defamed)
[![docs](https://docs.rs/defamed/badge.svg)](https://docs.rs/defamed)

</div>

## Quick start


## Introduction
Python allows users to call functions with named and default parameters.

<details>

<summary>Python example</summary>

```py
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

</details>

This macro generates a function macro that accepts positional and named parameters.
Parameters tagged with default values can be omitted.

```rust
// this function
#[defamed::defamed]
fn some_function(
    sign: bool,
    value: i32,
    // for types that implement std::default::Default
    #[def]
    add: i32,
    // any const expression can be a default
    #[def(2 + 1 - 2)]
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

## Parameter passing
The macro accepts parameters in any permutation as long as the following conditions are met:
- positional parameter order follows the original function signature
- all positional parameters are passed first
- named parameters come after all positional parameters
- named parameters can be included in any order
- default parameters are passed last
- default parameters can be excluded

<details>

<summary>Example</summary>

```rust
/// Add/sub 2 numbers, then take the absolute value, if applicable
#[defamed::defamed]
fn pos_and_def(
    lhs: i32,
    rhs: i32,
    #[def(true)]
    add: bool,
    #[def]
    abs_val: bool
) -> i32 {
    let inter = if add {lhs + rhs} else {lhs - rhs};
    if abs_val {inter.abs()} else {inter}
}

// original fn
assert_eq!(20, pos_and_def(5, 15, true, false));

// all positional
assert_eq!(20, pos_and_def!(5, 15, true, false));
// all named
assert_eq!(20, pos_and_def(lhs=5, rhs=15, add=true, abs_val=false));
// all named, in any order, defaults last
assert_eq!(20, pos_and_def(rhs=15, lhs=5, abs_val=false, add=true));
// defaults excluded
assert_eq!(20, pos_and_def!(5, 15));
// defaults excluded, positional in any order
assert_eq!(20, pos_and_def!(rhs=15, lhs=5));
// some positional, some named
assert_eq!(20, pos_and_def!(5, rhs=15));

// overriding first default parameter as positional
assert_eq!(20, pos_and_def!(25, 5, false));
// overriding second default parameter as named
assert_eq!(20, pos_and_def!(5, -25, abs_val=true));
```

</details>

## Macro scope
Macros generated by `defamed` can be exported and used by other crates.

The function macro generated by this crate needs to resolve to a valid function call at compile time.
So, the visibility of the source function determines how it ends up being called.

### Private
For private functions, the macro resolves the call directly.
> ```rust
> #[defamed::defamed]
> fn local_scope() {}
> // macro resolves to:
> local_scope!() => local_scope()
> ```

### Crate
Functions with crate level visibility and below are called with their corresponding fully qualified path relative to the crate root.
The macro will require a path to the function relative to the crate root.

> ```rust
> pub mod inner {
>     #[defamed::defamed(inner)]
>     pub(crate) fn crate_scope() {}
> }
> // macro resolves to:
> crate_scope!() => crate::inner::crate_scope()
> ```

### Public
The call site of public functions differs whether a function is called within its own crate or by another crate.

For the macro to resolve local and external calls, crate-local calls need to include `crate:` in the function call signature.

> ```rust
> pub mod inner {
>     #[defamed::defamed(inner)]
>     pub fn exported_function() {}
> }
> // for invocations within crate:
> inner::exported_function!(crate:) => crate::inner::exported_function()
> // for invocations by other crates:
> CRATE_NAME::inner::exported_function!() => CRATE_NAME::inner::exported_function()
> ```

For top-level exported functions, add `crate` as a path to the proc-macro.
> ```rust
> #[defamed::defamed(crate)]
> pub fn exported_root_function() {}
> // invocations are the same as in the above example.
> ```

## Benefits
- Better ergonomics
- More clarity during code reviews
- Seamless addition of default parameters to existing functions without breaking compatibility

## Limitations
This proc-macro currently works for standalone functions defined outside of an `impl` block. A fix is in the works.

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
