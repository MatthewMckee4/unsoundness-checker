# `typing.cast` used

Detects usage of `typing.cast()` function calls, which can lead to runtime errors by bypassing type checking without any runtime validation.

`typing.cast()` tells the type checker to treat a value as a specific type without performing any runtime checks. Type checkers trust casts completely, so incorrect casts bypass all type safety guarantees.

## What gets flagged

```python
from typing import cast

def foo() -> int:
    return cast(int, "hello")
```

## Better alternative

Instead of using `cast()`, use `assert isinstance()` to validate types at runtime:

```python
def get_value() -> int | str:
    return "hello"

result = get_value()
assert isinstance(result, int)
# Now we know it's safe - the assertion will catch incorrect types
```

This provides actual runtime safety instead of just lying to the type checker.

## What is okay

If the type of the value you are casting is assignable to the target type, it is okay to use `typing.cast()`.

```py
from typing import cast

def foo() -> int:
    return cast(int, 1)
```
