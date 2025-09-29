# Invalid overload implementation

## Valid Overloads

```py
from typing import overload

@overload
def process(x: int) -> str: ...
@overload
def process(x: str) -> int: ...
def process(x: int | str) -> str | int:
    if isinstance(x, int):
        return str(x)
    return len(x)
```

```py
from typing import overload

@overload
def convert(x: list[int]) -> tuple[int, ...]: ...
@overload
def convert(x: dict[str, int]) -> list[int]: ...
def convert(x: list[int] | dict[str, int]) -> tuple[int, ...] | list[int]:
    if isinstance(x, list):
        return tuple(x)
    return list(x.values())
```

```py
from typing import overload, Union

@overload
def handle(x: int, y: str) -> bool: ...
@overload
def handle(x: str, y: int) -> float: ...
def handle(x: Union[int, str], y: Union[str, int]) -> Union[bool, float]:
    if isinstance(x, int) and isinstance(y, str):
        return True
    return 1.0
```

## Invalid Overloads

We emit diagnostics for return types that are not assignable to any of the overload return types.

```py
from typing import overload

@overload
def foo(x: int) -> str: ...
@overload
def foo(x: str) -> int: ...
def foo(x: int | str) -> int | str:
    return x
```


```py
from typing import overload

@overload
def bar(x: int) -> str: ...
@overload
def bar(x: str) -> int: ...
def bar(x: int | str) -> object:
    return b""
```

```py
from typing import overload

@overload
def bar(x: int) -> str: ...
@overload
def bar(x: str) -> int: ...
def bar(x: int | str) -> list[str]:
    return ["invalid"]
```

Though, we currently emit no diagnostic for the following:

```py
from typing import overload

@overload
def bar(x: int) -> str: ...
@overload
def bar(x: str) -> int: ...
def bar(x: int | str) -> object:
    return 1
```
