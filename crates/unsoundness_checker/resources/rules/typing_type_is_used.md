# `typing.TypeIs` used

Detects usage of `typing.TypeIs` in return type annotations, which can lead to runtime errors if the type narrowing function is implemented incorrectly.

`typing.TypeIs` is used for type narrowing functions that tell type checkers to narrow the type of a variable. Type checkers trust TypeIs functions completely, so incorrect implementations bypass all type safety guarantees.

## What gets flagged

```python
from typing_extensions import TypeIs

def is_int(x: object) -> TypeIs[int]:
    return True

value = "hello"
if is_int(value):
    # Type checker thinks value is int, but it's actually str
    result = value + 1
```

## Better alternative

Instead of using `TypeIs`, use `isinstance()` checks directly in your code:

This provides actual runtime safety instead of just telling the type checker to trust your narrowing logic.

## What is okay

If your `TypeIs` function correctly validates the type at runtime, it should be safe. However, since there's no way for type checkers to verify the implementation matches the signature, it's easy to introduce bugs.

```python
from typing_extensions import TypeIs

def is_int(x: object) -> TypeIs[int]:
    return isinstance(x, int)
```
