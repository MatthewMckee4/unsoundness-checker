# If TYPE_CHECKING used

Detects usage of `if TYPE_CHECKING:` blocks from the `typing` module.

`TYPE_CHECKING` is `False` at runtime but `True` during static type checking. This creates a disconnect between type checking and runtime behavior that can cause errors.

## Why this is bad

The most dangerous pattern is using `if TYPE_CHECKING` with an `else` clause where signatures don't match:

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    def get_value() -> int:
        ...
else:
    def get_value() -> str:
        return "hello"
```


```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    TIMEOUT: int = 30
else:
    TIMEOUT: str = "30"

```
