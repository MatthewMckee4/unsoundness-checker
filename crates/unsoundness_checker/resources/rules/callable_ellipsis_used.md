# Ellipsis in Callable Type Annotations

Detects usage of `...` (ellipsis) in the first argument of `Callable` type annotations, which bypasses parameter type checking.

Using `Callable[..., ReturnType]` indicates that the callable accepts any number of arguments of any type. The type checker cannot verify argument types or counts, which can lead to runtime errors.

## What gets flagged

```python
from typing import Callable

def foo(callback: Callable[..., int]) -> int:
    return callback("wrong", "types")

def bar(x: int) -> int:
    return x

# This passes type checking but fails at runtime.
foo(bar)
```
