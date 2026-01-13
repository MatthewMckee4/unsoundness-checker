# Invalid overload implementation - Extensive Tests

This file contains extensive tests for the invalid overload implementation rule, covering various scenarios where implementations don't match overload signatures.

## What gets flagged

### Basic object return type

```python
from typing import overload

@overload
def bar(x: int) -> str: ...
@overload
def bar(x: str) -> int: ...
def bar(x: int | str) -> object:
    return b""
```

### Wrong concrete return type

```python
from typing import overload

@overload
def foo(x: int) -> str: ...
@overload
def foo(x: str) -> int: ...
def foo(x: int | str) -> bytes:
    return b"bytes"
```

### Return type too broad

```python
from typing import overload

@overload
def process(x: int) -> list[int]: ...
@overload
def process(x: str) -> list[str]: ...
def process(x: int | str) -> list:
    return []
```

### Multiple overloads with object

```python
from typing import overload

@overload
def multi(x: int, y: int) -> int: ...
@overload
def multi(x: str, y: str) -> str: ...
@overload
def multi(x: bool, y: bool) -> bool: ...
def multi(x: int | str | bool, y: int | str | bool) -> object:
    return None
```

### Class method with wrong return

```python
from typing import overload

class Processor:
    @overload
    def process(self, x: int) -> str: ...
    @overload
    def process(self, x: str) -> int: ...
    def process(self, x: int | str) -> object:
        return []
```

### Classmethod with object return

```python
from typing import overload

class Factory:
    @overload
    @classmethod
    def create(cls, x: int) -> "Factory": ...
    @overload
    @classmethod
    def create(cls, x: str) -> "Factory": ...
    @classmethod
    def create(cls, x: int | str) -> object:
        return object()
```

### Staticmethod with wrong return

```python
from typing import overload

class Utils:
    @overload
    @staticmethod
    def convert(x: int) -> str: ...
    @overload
    @staticmethod
    def convert(x: str) -> int: ...
    @staticmethod
    def convert(x: int | str) -> object:
        return None
```

### Generic overload with object

```python
from typing import overload, TypeVar

T = TypeVar('T')

@overload
def wrap(x: int) -> list[int]: ...
@overload
def wrap(x: str) -> list[str]: ...
def wrap(x: int | str) -> object:
    return []
```

### Overload with None not in union

```python
from typing import overload

@overload
def maybe(x: int) -> int: ...
@overload
def maybe(x: str) -> str: ...
def maybe(x: int | str) -> int | str | None:
    return None
```

### Dict return instead of specific

```python
from typing import overload

@overload
def get_data(x: int) -> dict[str, int]: ...
@overload
def get_data(x: str) -> dict[str, str]: ...
def get_data(x: int | str) -> dict:
    return {}
```

### Tuple return too generic

```python
from typing import overload

@overload
def pair(x: int) -> tuple[int, int]: ...
@overload
def pair(x: str) -> tuple[str, str]: ...
def pair(x: int | str) -> tuple:
    return (x, x)
```

### Any as implementation return

```python
from typing import overload, Any

@overload
def process(x: int) -> str: ...
@overload
def process(x: str) -> int: ...
def process(x: int | str) -> Any:
    return x
```

### Overload with list vs sequence

```python
from typing import overload, Sequence

@overload
def collect(x: int) -> list[int]: ...
@overload
def collect(x: str) -> list[str]: ...
def collect(x: int | str) -> Sequence:
    return []
```

### Overload return not matching callable

```python
from typing import overload, Callable

@overload
def maker(x: int) -> Callable[[], int]: ...
@overload
def maker(x: str) -> Callable[[], str]: ...
def maker(x: int | str) -> object:
    return lambda: None
```

### Protocol instead of concrete

```python
from typing import overload, Protocol

class Stringable(Protocol):
    def __str__(self) -> str: ...

@overload
def fmt(x: int) -> int: ...
@overload
def fmt(x: str) -> str: ...
def fmt(x: int | str) -> Stringable:
    return x
```

### Inner functions

We must ensure we do not emit diagnostics for returns statements inside inner functions.

Regression: <https://github.com/MatthewMckee4/unsoundness-checker/issues/71>

```python
from typing import overload

@overload
def foo() -> str: ...
@overload
def foo() -> int: ...
def foo() -> str | int:
    def inner():
        return []
    return 1

 @overload
 def bar() -> str: ...
 @overload
 def bar() -> int: ...
 def bar() -> str | int:
    def inner1():
        def inner2():
            return []
        return []
    return 3
```

## What we can't catch

### Valid but complex implementation

```python
from typing import overload

@overload
def baz(x: int) -> str: ...
@overload
def baz(x: str) -> int: ...
def baz(x: int | str) -> int | str:
    return x
```

### Generic function loses type info

```python
from typing import overload, TypeVar

T = TypeVar("T")

def custom_copy(x: T) -> T:
    return x

@overload
def baz(x: list[int]) -> str: ...
@overload
def baz(x: str) -> int: ...
def baz(x: list[int] | str) -> int | str:
    y = custom_copy(x)
    if isinstance(y, list):
        return ""
    else:
        return 1
```
