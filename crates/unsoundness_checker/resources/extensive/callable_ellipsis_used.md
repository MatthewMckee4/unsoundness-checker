# Ellipsis in Callable Type Annotations - Extensive Tests

This file contains extensive tests for the callable ellipsis rule, covering various scenarios where ellipsis bypasses parameter checking.

## What gets flagged

### Basic ellipsis in callback

```python
from typing import Callable

def foo(callback: Callable[..., int]) -> int:
    return callback("wrong", "types")

def bar(x: int) -> int:
    return x

foo(bar)
```

### Ellipsis with multiple callbacks

```python
from typing import Callable

def process(
    handler1: Callable[..., str],
    handler2: Callable[..., int]
) -> tuple[str, int]:
    return handler1(), handler2()
```

### Ellipsis in variable annotation

```python
from typing import Callable

processor: Callable[..., None] = lambda x, y: print(x, y)
```

### Ellipsis in class attribute

```python
from typing import Callable

class Handler:
    callback: Callable[..., str]

    def __init__(self, cb: Callable[..., str]):
        self.callback = cb
```

### Ellipsis in return type

```python
from typing import Callable

def get_processor() -> Callable[..., int]:
    return lambda *args, **kwargs: 42
```

### Ellipsis with generic return

```python
from typing import Callable, TypeVar

T = TypeVar('T')

def wrap(func: Callable[..., T]) -> Callable[..., T]:
    return func
```

### Ellipsis in list of callables

```python
from typing import Callable

handlers: list[Callable[..., None]] = [
    lambda: None,
    lambda x: print(x),
    lambda x, y: print(x, y)
]
```

### Ellipsis in dict values

```python
from typing import Callable

processors: dict[str, Callable[..., int]] = {
    "add": lambda x, y: x + y,
    "mul": lambda x, y: x * y
}
```

### Ellipsis with Optional

```python
from typing import Callable, Optional

maybe_handler: Optional[Callable[..., str]] = None
```

### Ellipsis in Union

```python
from typing import Callable, Union

handler: Union[Callable[..., int], Callable[..., str]] = lambda: 42
```

### Ellipsis in method signature

```python
from typing import Callable

class Processor:
    def process(self, func: Callable[..., int]) -> int:
        return func()
```

### Ellipsis in nested callable

```python
from typing import Callable

def higher_order(
    maker: Callable[..., Callable[..., int]]
) -> Callable[..., int]:
    return maker()
```

### Ellipsis with Protocol

```python
from typing import Callable, Protocol

class HandlerProtocol(Protocol):
    callback: Callable[..., str]
```

### Ellipsis in lambda type

```python
from typing import Callable

make_handler: Callable[[str], Callable[..., int]] = lambda name: lambda *args: 42
```

### Multiple ellipsis in same function

```python
from typing import Callable

def multi_handler(
    on_success: Callable[..., None],
    on_error: Callable[..., None],
    on_complete: Callable[..., None]
) -> None:
    pass
```
