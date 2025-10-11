# Invalid overload implementation

## Valid Overloads

```py
from typing import overload

@overload
def foo(x: int) -> str: ...
@overload
def foo(x: str) -> int: ...
def foo(x: int | str) -> str | int:
    if isinstance(x, int):
        return str(x)
    return len(x)
```

```py
from typing import overload

@overload
def foo(x: int, y: str) -> bool: ...
@overload
def foo(x: str, y: int) -> float: ...
def foo(x: int | str, y: str | int) -> bool | float:
    if isinstance(x, int) and isinstance(y, str):
        return True
    return 1.0
```

## Invalid Overloads Implementations

### What we can catch

We emit diagnostics for return types that are not assignable to the union of the overload return types.

The main issue that type checkers won't always pick up on is that the implementation return type can simply be `object` which everything is assignable to.

```py
from typing import overload

@overload
def bar(x: int) -> str: ...
@overload
def bar(x: str) -> int: ...
def bar(x: int | str) -> object:
    return b""
```

A side note is that if you changed the return type of `bar` implementation to `int | str`, then most type checkers would catch this error.

```py
from typing import overload

@overload
def bar(x: int) -> str: ...
@overload
def bar(x: str) -> int: ...
def bar(x: int | str) -> int | str:
    return b""
```

### What we can't catch

Due to more complex examples, we currently can't catch all invalid overload implementations.

The idea for the implementation of this was that at each return statement if all input variables had been narrowed to the types of the matching overload statement, then this would be a valid implementation.

This is a obvious incorrect implementation.

```py

from typing import overload

@overload
def baz(x: int) -> str: ...
@overload
def baz(x: str) -> int: ...
def baz(x: int | str) -> int | str:
    return x
```

This is a more complex example which makes it very difficult to catch any errors. this is a valid implementation, but because we lost information about the use of `x`,
we cannot make any assumptions about if this is valid or not.

And because we don't want to emit false positives, we will emit anything here.

```py
from typing import overload, TypeVar

T = TypeVar("T")

def custom_copy(x: T) -> T:
    return x

@overload
def baz(x: list[int]) -> str: ...
@overload
def baz(x: str) -> int: ...
def baz(x: list[int] | str) -> int | str:
    # Some complex function that loses information about the type of x, and how it is used
    y = custom_copy(x)
    if isinstance(y, list):
        return ""
    else:
        return 1
```
