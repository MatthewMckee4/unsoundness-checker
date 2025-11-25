# If TYPE_CHECKING used - Extensive Tests

This file contains extensive tests for the TYPE_CHECKING rule, covering various scenarios where runtime and type-time behavior diverge.

## What gets flagged

### Basic TYPE_CHECKING with else

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    def get_value() -> int:
        ...
else:
    def get_value() -> str:
        return "hello"
```

### TYPE_CHECKING for imports

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from expensive_module import ExpensiveClass
else:
    ExpensiveClass = None
```

### TYPE_CHECKING with class definition

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    class MyClass:
        value: int
else:
    class MyClass:
        value: str
```

### TYPE_CHECKING with variable annotation

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    value: int = 0
else:
    value: str = "zero"
```

### Nested TYPE_CHECKING blocks

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    if True:
        x: int = 0
else:
    x: str = "zero"
```

### TYPE_CHECKING in class body

```python
from typing import TYPE_CHECKING

class Container:
    if TYPE_CHECKING:
        data: int
    else:
        data: str
```

### TYPE_CHECKING with function overload

```python
from typing import TYPE_CHECKING, overload

if TYPE_CHECKING:
    @overload
    def process(x: int) -> str: ...
    @overload
    def process(x: str) -> int: ...

def process(x):
    return x
```

### TYPE_CHECKING with type alias

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    Data = dict[str, int]
else:
    Data = dict[str, str]
```

### TYPE_CHECKING with multiple definitions

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    def foo() -> int: ...
    def bar() -> str: ...
else:
    def foo() -> str:
        return "foo"
    def bar() -> int:
        return 42
```

### TYPE_CHECKING with global variable

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    CONSTANT: int = 0
else:
    CONSTANT: str = "zero"
```

### TYPE_CHECKING in module level

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import json as json_module
else:
    json_module = None
```

### TYPE_CHECKING with method

```python
from typing import TYPE_CHECKING

class Handler:
    if TYPE_CHECKING:
        def handle(self) -> int: ...
    else:
        def handle(self) -> str:
            return "handled"
```

### TYPE_CHECKING with conditional import

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Sequence
    def process(items: Sequence[int]) -> None: ...
else:
    def process(items):
        pass
```

### TYPE_CHECKING with __future__ import

```python
from __future__ import annotations
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    ComplexType = dict[str, list[int]]
else:
    ComplexType = dict
```

### Multiple TYPE_CHECKING blocks

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    x: int = 0
else:
    x: str = "zero"

if TYPE_CHECKING:
    y: bool = True
else:
    y: int = 1
```
