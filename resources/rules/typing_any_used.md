# `typing.Any` used

Detects usage of `typing.Any` in type annotations, which can lead to runtime errors by bypassing type checking.

When you use `Any`, you're essentially telling the type checker "this could be anything," which defeats the purpose of having types in the first place. This can cause bugs that would otherwise be caught at development time.

## What gets flagged

We currently detect `Any` in these places:

### Function parameters

```python
from typing import Any

def process_data(data: Any) -> str:
    return str(data)
```

### Return types

```python
from typing import Any

def get_user_data() -> Any:
    return {"name": "John", "age": 30}
```

### Variable assignments

```python
from typing import Any

user_info: Any = get_user()
```

## Why this matters

Using `Any` removes type safety. For example:

```python
from typing import Any

def calculate_total(items: Any) -> int:
    return sum(item for item in items)
```

Instead, be specific about what you expect:

```python
def calculate_total(items: list[int]) -> int:
    return sum(item for item in items)
```

## Other Examples

The checker also finds `Any` nested in other types:

```python
from typing import Any

def process(data: dict[str, Any]) -> None: pass
def get_items() -> list[Any]: pass
user_data: dict[str, Any] = {}
```
