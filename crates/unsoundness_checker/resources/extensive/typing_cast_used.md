# `typing.cast` used - Extensive Tests

This file contains extensive tests for the `typing.cast` rule, covering various unsafe casting scenarios.

## What gets flagged

### Basic unsafe cast

```python
from typing import cast

def foo() -> int:
    return cast(int, "hello")
```

### Casting between incompatible types

```python
from typing import cast

def get_value() -> str:
    value = cast(str, 123)
    return value
```

### Casting in variable assignment

```python
from typing import cast

number = cast(int, "not a number")
```

### Casting with complex types

```python
from typing import cast, List, Dict

data = cast(List[int], ["a", "b", "c"])
mapping = cast(Dict[str, int], {"key": "not_an_int"})
```

### Nested casts

```python
from typing import cast

value = cast(int, cast(str, 42))
```

### Casting with generics

```python
from typing import cast, TypeVar

T = TypeVar('T')

def unsafe_cast(value: object) -> T:
    return cast(T, value)
```

### Casting None to non-optional

```python
from typing import cast

value: int = cast(int, None)
```

### Casting in return statements

```python
from typing import cast

def get_number() -> int:
    result = get_string_value()
    return cast(int, result)

def get_string_value() -> str:
    return "hello"
```

### Casting with union types

```python
from typing import cast, Union

def process(value: Union[int, str]) -> int:
    return cast(int, value)
```

### Casting in list comprehension

```python
from typing import cast

numbers = [cast(int, x) for x in ["1", "2", "3"]]
```

### Casting function return values

```python
from typing import cast, Callable

def wrapper() -> int:
    func: Callable[[], str] = lambda: "hello"
    return cast(int, func())
```

### Casting with custom classes

```python
from typing import cast

class Dog:
    def bark(self) -> str:
        return "woof"

class Cat:
    def meow(self) -> str:
        return "meow"

def wrong_pet() -> Dog:
    cat = Cat()
    return cast(Dog, cat)
```

### Multiple casts in same function

```python
from typing import cast

def multi_cast():
    a = cast(int, "hello")
    b = cast(str, 123)
    c = cast(bool, "yes")
    return a, b, c
```

## What is okay

### Safe cast with compatible types

```python
from typing import cast

def foo() -> int:
    return cast(int, 1)
```

### Narrowing cast

```python
from typing import cast

def process(value: object) -> int:
    if isinstance(value, int):
        return cast(int, value)
    return 0
```
