# `typing.overload` used

Detects use of overloaded functions.

Overloaded functions can often lead to runtime errors if the implementations are not consistent with the overload definitions.

## Examples

Here is an example of an overloaded function that is implemented correctly:

```py
from typing import overload

@overload
def add(a: int, b: int) -> int: ...

@overload
def add(a: float, b: float) -> float: ...

def add(a: float, b: float) -> float | int:
    return a + b
```

Whereas this one is not:

```py
from typing import overload

@overload
def foo(x: int) -> str: ...

@overload
def foo(x: str) -> int: ...

def foo(x: int | str) -> str | int:
    return x
```

It is very hard for us to detect when an overloaded function is implemented incorrectly, so with this rule we can try to eliminate all uses of overloaded functions.
