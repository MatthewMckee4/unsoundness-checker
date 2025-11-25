# `typing.TypeIs` used - Extensive Tests

This file contains extensive tests for the TypeIs rule, covering various scenarios where TypeIs can bypass type safety.

## What gets flagged

### Basic incorrect TypeIs

```python
from typing_extensions import TypeIs

def is_int(x: object) -> TypeIs[int]:
    return True

value = "hello"
if is_int(value):
    result = value + 1
```

### TypeIs always returning False

```python
from typing_extensions import TypeIs

def is_string(x: object) -> TypeIs[str]:
    return False

value = "actual_string"
if is_string(value):
    length = len(value)
```

### TypeIs with wrong check

```python
from typing_extensions import TypeIs

def is_list_of_ints(x: object) -> TypeIs[list[int]]:
    return isinstance(x, list)

value = ["a", "b", "c"]
if is_list_of_ints(value):
    total = sum(value)
```

### Multiple TypeIs functions

```python
from typing_extensions import TypeIs

def is_int(x: object) -> TypeIs[int]:
    return True

def is_str(x: object) -> TypeIs[str]:
    return True

value: object = 42
if is_str(value):
    upper = value.upper()
```

### TypeIs in class method

```python
from typing_extensions import TypeIs

class Validator:
    @staticmethod
    def is_positive(x: object) -> TypeIs[int]:
        return True

value = -5
if Validator.is_positive(value):
    result = value + 10
```

### TypeIs with generic

```python
from typing_extensions import TypeIs
from typing import TypeVar

T = TypeVar('T')

def is_list(x: object) -> TypeIs[list[T]]:
    return True
```

### TypeIs narrowing to wrong subtype

```python
from typing_extensions import TypeIs

class Animal:
    pass

class Dog(Animal):
    def bark(self) -> None:
        pass

def is_dog(x: Animal) -> TypeIs[Dog]:
    return True

cat = Animal()
if is_dog(cat):
    cat.bark()
```

### TypeIs with union

```python
from typing_extensions import TypeIs

def is_int_or_str(x: object) -> TypeIs[int | str]:
    return isinstance(x, bool)
```

### TypeIs in conditional

```python
from typing_extensions import TypeIs

def maybe_int(x: object, strict: bool) -> TypeIs[int]:
    return True if strict else False
```

### TypeIs with complex type

```python
from typing_extensions import TypeIs

def is_dict_str_int(x: object) -> TypeIs[dict[str, int]]:
    return isinstance(x, dict)

value = {"key": "not_int"}
if is_dict_str_int(value):
    num = value["key"] + 1
```

### Nested TypeIs checks

```python
from typing_extensions import TypeIs

def is_int(x: object) -> TypeIs[int]:
    return True

def is_positive_int(x: object) -> TypeIs[int]:
    return is_int(x)
```

### TypeIs with Protocol

```python
from typing_extensions import TypeIs, Protocol

class Drawable(Protocol):
    def draw(self) -> None: ...

def is_drawable(x: object) -> TypeIs[Drawable]:
    return True
```

### TypeIs in lambda (if possible)

```python
from typing_extensions import TypeIs

checker: callable = lambda x: True  # Can't actually type this as TypeIs
```

### TypeIs with Literal

```python
from typing_extensions import TypeIs
from typing import Literal

def is_success(x: str) -> TypeIs[Literal["success"]]:
    return True
```

### TypeIs narrowing from Any

```python
from typing import Any
from typing_extensions import TypeIs

def is_int(x: Any) -> TypeIs[int]:
    return True
```

### Multiple parameters with TypeIs

```python
from typing_extensions import TypeIs

def are_both_ints(x: object, y: object) -> TypeIs[int]:
    return True
```

## What is okay

### Correct TypeIs implementation

```python
from typing_extensions import TypeIs

def is_int(x: object) -> TypeIs[int]:
    return isinstance(x, int)

value: object = 42
if is_int(value):
    result = value + 1
```

### TypeIs with proper validation

```python
from typing_extensions import TypeIs

def is_list_of_ints(x: object) -> TypeIs[list[int]]:
    return isinstance(x, list) and all(isinstance(i, int) for i in x)

value: object = [1, 2, 3]
if is_list_of_ints(value):
    total = sum(value)
```

### TypeIs narrowing with isinstance

```python
from typing_extensions import TypeIs

class Animal:
    pass

class Dog(Animal):
    def bark(self) -> None:
        pass

def is_dog(x: Animal) -> TypeIs[Dog]:
    return isinstance(x, Dog)
```
