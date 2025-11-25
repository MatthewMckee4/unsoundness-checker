# `typing.overload` used - Extensive Tests

This file contains extensive tests for the `typing.overload` rule, covering various scenarios where overloads can be misimplemented.

## What gets flagged

### Basic incorrect overload implementation

```python
from typing import overload

@overload
def foo(x: int) -> str: ...

@overload
def foo(x: str) -> int: ...

def foo(x: int | str) -> str | int:
    return x
```

### Overload with wrong return type

```python
from typing import overload

@overload
def process(x: int) -> str: ...

@overload
def process(x: str) -> int: ...

def process(x: int | str) -> bool:
    return True
```

### Multiple parameter overloads

```python
from typing import overload

@overload
def combine(a: int, b: int) -> int: ...

@overload
def combine(a: str, b: str) -> str: ...

def combine(a: int | str, b: int | str) -> int | str:
    if isinstance(a, int) and isinstance(b, int):
        return a + b
    return str(a) + str(b)
```

### Overload with optional parameters

```python
from typing import overload

@overload
def create(name: str) -> dict[str, str]: ...

@overload
def create(name: str, age: int) -> dict[str, str | int]: ...

def create(name: str, age: int | None = None) -> dict[str, str | int]:
    if age is None:
        return {"name": name}
    return {"name": name, "age": age}
```

### Method overloads

```python
from typing import overload

class DataProcessor:
    @overload
    def process(self, data: int) -> str: ...

    @overload
    def process(self, data: str) -> int: ...

    def process(self, data: int | str) -> str | int:
        return data
```

### Classmethod overloads

```python
from typing import overload

class Factory:
    @overload
    @classmethod
    def create(cls, value: int) -> "Factory": ...

    @overload
    @classmethod
    def create(cls, value: str) -> "Factory": ...

    @classmethod
    def create(cls, value: int | str) -> "Factory":
        return cls()
```

### Staticmethod overloads

```python
from typing import overload

class Utilities:
    @overload
    @staticmethod
    def convert(x: int) -> str: ...

    @overload
    @staticmethod
    def convert(x: str) -> int: ...

    @staticmethod
    def convert(x: int | str) -> int | str:
        return x
```

### Overload with generic types

```python
from typing import overload, TypeVar

T = TypeVar('T')

@overload
def wrap(value: int) -> list[int]: ...

@overload
def wrap(value: str) -> list[str]: ...

def wrap(value: int | str) -> list[int] | list[str]:
    return [value]
```

### Nested overloads in class

```python
from typing import overload

class Calculator:
    @overload
    def compute(self, x: int, y: int) -> int: ...

    @overload
    def compute(self, x: float, y: float) -> float: ...

    def compute(self, x: int | float, y: int | float) -> int | float:
        return x + y
```

### Overload with complex return types

```python
from typing import overload

@overload
def fetch(id: int) -> dict[str, int]: ...

@overload
def fetch(id: str) -> dict[str, str]: ...

def fetch(id: int | str) -> dict[str, int] | dict[str, str]:
    if isinstance(id, int):
        return {"id": id}
    return {"name": id}
```

### Overload with None return

```python
from typing import overload

@overload
def maybe_process(x: int) -> int: ...

@overload
def maybe_process(x: None) -> None: ...

def maybe_process(x: int | None) -> int | None:
    return x
```

### Multiple overloads same signature

```python
from typing import overload, Literal

@overload
def get_value(key: Literal["name"]) -> str: ...

@overload
def get_value(key: Literal["age"]) -> int: ...

@overload
def get_value(key: str) -> str | int: ...

def get_value(key: str) -> str | int:
    if key == "name":
        return "John"
    elif key == "age":
        return 30
    return "unknown"
```

### Overload with callbacks

```python
from typing import overload, Callable

@overload
def process_with(handler: Callable[[int], str]) -> str: ...

@overload
def process_with(handler: Callable[[str], int]) -> int: ...

def process_with(handler: Callable[[int], str] | Callable[[str], int]) -> str | int:
    return handler(1) if callable(handler) else ""
```
