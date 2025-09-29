# Invalid overload implementation

```py
from typing import overload

@overload
def foo(x: int) -> str: ...
@overload
def foo(x: str) -> int: ...
def foo(x: int | str) -> int | str:
    return x
```

We emit diagnostics for return types that are not assignable to any of the overload return types.

```py
from typing import overload

@overload
def bar(x: int) -> str: ...
@overload
def bar(x: str) -> int: ...
def bar(x: int | str) -> object:
    return b""
```
