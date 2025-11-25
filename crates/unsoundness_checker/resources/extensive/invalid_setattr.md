# Invalid `setattr()` call - Extensive Tests

This file contains extensive tests for the invalid setattr rule, covering various scenarios where setattr bypasses type safety.

## What gets flagged

### Basic setattr with wrong type

```python
class Foo:
    def __init__(self) -> None:
        self.x: str = "hello"

foo = Foo()
setattr(foo, "x", 1)
```

### Setattr with multiple attributes

```python
class Person:
    def __init__(self) -> None:
        self.name: str = "John"
        self.age: int = 30

person = Person()
setattr(person, "name", 123)
setattr(person, "age", "thirty")
```

### Setattr in method

```python
class Config:
    def __init__(self) -> None:
        self.value: int = 0

    def update(self, val: str) -> None:
        setattr(self, "value", val)
```

### Setattr with computed attribute name

```python
class Dynamic:
    def __init__(self) -> None:
        self.data: dict[str, int] = {}

obj = Dynamic()
attr_name = "data"
setattr(obj, attr_name, "not a dict")
```

### Setattr on different instances

```python
class Container:
    value: str = "default"

c1 = Container()
c2 = Container()
setattr(c1, "value", 123)
setattr(c2, "value", True)
```

### Setattr in loop

```python
class Record:
    field1: int = 0
    field2: str = ""

record = Record()
for attr, val in [("field1", "wrong"), ("field2", 42)]:
    setattr(record, attr, val)
```

### Setattr with class variable

```python
class Settings:
    timeout: int = 30

setattr(Settings, "timeout", "not_a_number")
```

### Setattr creating new attribute

```python
class Base:
    existing: int = 1

obj = Base()
setattr(obj, "new_attr", "value")
```

### Nested setattr calls

```python
class Outer:
    def __init__(self) -> None:
        self.inner: "Inner" = Inner()

class Inner:
    value: int = 0

outer = Outer()
setattr(outer.inner, "value", "wrong")
```

### Setattr with property

```python
class Managed:
    def __init__(self) -> None:
        self._value: int = 0

    @property
    def value(self) -> int:
        return self._value

    @value.setter
    def value(self, val: int) -> None:
        self._value = val

obj = Managed()
setattr(obj, "value", "not_int")
```

### Setattr in inheritance

```python
class Parent:
    parent_attr: str = "parent"

class Child(Parent):
    child_attr: int = 0

child = Child()
setattr(child, "parent_attr", 123)
setattr(child, "child_attr", "wrong")
```

### Setattr with generic class

```python
from typing import Generic, TypeVar

T = TypeVar('T')

class Box(Generic[T]):
    def __init__(self, value: T) -> None:
        self.value: T = value

box = Box[int](42)
setattr(box, "value", "not_int")
```

### Setattr in __init__

```python
class Initialize:
    x: int

    def __init__(self) -> None:
        setattr(self, "x", "wrong_type")
```

### Conditional setattr

```python
class Conditional:
    flag: bool = False

obj = Conditional()
condition = True
if condition:
    setattr(obj, "flag", "not_bool")
```

### Setattr with __dict__

```python
class Direct:
    value: int = 0

obj = Direct()
setattr(obj, "__dict__", "not_a_dict")
```
