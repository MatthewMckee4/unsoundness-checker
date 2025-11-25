# Accessing `__defaults__` attribute - Extensive Tests

This file contains extensive tests for the invalid function defaults rule, covering various scenarios where modifying `__defaults__` bypasses type checking.

## What gets flagged

### Basic wrong type default

```python
def foo(x: int = 1) -> int:
    return x

foo.__defaults__ = ("string",)
```

### Setting defaults to None

```python
def foo(x: int = 1) -> int:
    return x

foo.__defaults__ = None
```

### Multiple parameters wrong defaults

```python
def bar(x: int = 1, y: str = "string", z: bool = True) -> int:
    return x

bar.__defaults__ = (1, "string", "string")
```

### Clearing defaults

```python
def bar(x: int = 1, y: str = "string", z: bool = True) -> int:
    return x

bar.__defaults__ = None
```

### Too few defaults

```python
def bar(x: int = 1, y: str = "string", z: bool = True) -> int:
    return x

bar.__defaults__ = (1, "string")
```

### Wrong type in tuple

```python
def multi_param(a: int = 1, b: str = "hello", c: bool = True) -> None:
    pass

multi_param.__defaults__ = (1, 2, True)
```

### All wrong types

```python
def typed_func(x: int = 0, y: str = "") -> None:
    pass

typed_func.__defaults__ = ("wrong", 123)
```

### Class method defaults

```python
class MyClass:
    def method(self, x: int = 1) -> int:
        return x

MyClass.method.__defaults__ = ("string",)
```

### Instance method defaults

```python
class Container:
    def process(self, value: int = 0) -> int:
        return value

obj = Container()
obj.process.__defaults__ = ("not_int",)
```

### Static method defaults

```python
class Utilities:
    @staticmethod
    def compute(x: int = 1) -> int:
        return x

Utilities.compute.__defaults__ = ("string",)
```

### Class method with classmethod

```python
class Factory:
    @classmethod
    def create(cls, value: int = 0) -> "Factory":
        return cls()

Factory.create.__defaults__ = ("wrong",)
```

### Nested function defaults

```python
def outer():
    def inner(x: int = 1) -> int:
        return x
    inner.__defaults__ = ("string",)
    return inner
```

### Lambda defaults (if possible)

```python
func = lambda x=1: x
func.__defaults__ = ("string",)
```

### Complex type defaults

```python
from typing import List

def process_list(items: List[int] = []) -> List[int]:
    return items

process_list.__defaults__ = (["string", "list"],)
```

### Optional parameter wrong default

```python
from typing import Optional

def maybe_int(x: Optional[int] = None) -> Optional[int]:
    return x

maybe_int.__defaults__ = ("not_none_or_int",)
```

### Multiple defaults assignment

```python
def func(a: int = 1, b: str = "hello") -> None:
    pass

func.__defaults__ = (1, 2)
func.__defaults__ = ("wrong", "types")
```

## What is okay

### Matching string defaults

```python
def foo(x: str = "string") -> str:
    return x

foo.__defaults__ = ("another_string",)
```

### Extra matching defaults

```python
def foo(x: str = "string") -> str:
    return x

foo.__defaults__ = ("another_string", "another_string")
```

### Adding default to no-default parameter

```python
def bar(x: int): ...

bar.__defaults__ = (1,)
```

### Setting None for no defaults

```python
def baz(x: int): ...

baz.__defaults__ = None
```
