# `typing.overload` used

Detects use of overloaded functions.

Overloaded functions can often lead to runtime errors if the implementations are not consistent with the overload definitions.

We only emit a warning here as just using `typing.overload` will not necessarily lead to a runtime type error.

## What gets flagged

Here is an example of an overloaded function that is not implemented correctly, but type checkers will not emit diagnostics for this:

```py
from typing import overload

@overload
def foo(x: int) -> str: ...

@overload
def foo(x: str) -> int: ...

def foo(x: int | str) -> str | int:
    return x
```
