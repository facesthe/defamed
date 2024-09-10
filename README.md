# defame
Default, named and positional function arguments.

## Introduction
When writing code in Python, it is possible to do the following:

```python
def some_function(
    sign: bool,
    value: int,
    default: int = 10
) -> int:
    if sign:
        return value + default
    else:
        return 0 - value + default
```
