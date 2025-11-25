# `typing.Any` used - Extensive Tests

This file contains extensive tests for the `typing.Any` rule, covering various edge cases and scenarios.

## What gets flagged

### Function parameters with Any

```python
from typing import Any

def process_data(data: Any) -> str:
    return str(data)
```

### Multiple parameters with Any

```python
from typing import Any

def process_multiple(a: Any, b: Any, c: int) -> str:
    return str(a) + str(b) + str(c)
```

### Return types with Any

```python
from typing import Any

def get_user_data() -> Any:
    return {"name": "John", "age": 30}
```

### Variable assignments with Any

```python
from typing import Any

def get_user() -> dict[str, str | int]:
    return {"name": "John", "age": 30}

user_info: Any = get_user()
```

### Any in generic types

```python
from typing import Any, List, Dict

def process_list(items: List[Any]) -> None:
    pass

def process_dict(data: Dict[str, Any]) -> None:
    pass
```

### Any in nested generics

```python
from typing import Any, Dict, List

def complex_function(data: Dict[str, List[Any]]) -> None:
    pass
```

### Any in class attributes

```python
from typing import Any

class DataProcessor:
    data: Any

    def __init__(self, data: Any):
        self.data = data
```

### Any in method signatures

```python
from typing import Any

class Handler:
    def handle(self, request: Any) -> Any:
        return request
```

### Any in lambda functions

```python
from typing import Any, Callable

processor: Callable[[Any], Any] = lambda x: x
```

### Any with Optional

```python
from typing import Any, Optional

def maybe_process(data: Optional[Any]) -> str:
    if data is None:
        return "None"
    return str(data)
```

### Any with Union

```python
from typing import Any, Union

def flexible_handler(data: Union[str, Any]) -> str:
    return str(data)
```

### Any in tuple annotations

```python
from typing import Any, Tuple

def return_tuple() -> Tuple[int, Any, str]:
    return (1, "anything", "hello")
```

### Multiple Any usages in same function

```python
from typing import Any

def multi_any(a: Any, b: Any) -> Any:
    return a if a else b
```

### Any in property annotations

```python
from typing import Any

class Container:
    @property
    def value(self) -> Any:
        return self._value

    @value.setter
    def value(self, val: Any) -> None:
        self._value = val
```

### Any with type aliases

```python
from typing import Any

AnyData = Any

def process(data: AnyData) -> str:
    return str(data)
```
