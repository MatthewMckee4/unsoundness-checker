# Rules

## `invalid-overload-implementation`

<small>
Default level: `error`.
</small>

**What it does**

Checks for invalid overload implementation.

**Why is this bad?**

Invalid overload implementation can lead to runtime errors.

**Examples**

```python
from typing import overload

@overload
def foo(x: int) -> str: ...
@overload
def foo(x: str) -> int: ...
def foo(x: int | str) -> int | str:
    return x

foo("1")
```

[See more](rules/invalid_overload_implementation.md)

## `typing-any-used`

<small>
Default level: `error`.
</small>

**What it does**

Checks for usage of `typing.Any` in type annotations.

**Why is this bad?**

Using `typing.Any` in type annotations can lead to runtime errors.

**Examples**

```python
from typing import Any

def foo(x: Any) -> Any:
    return x + 1

foo("1")
```

[See more](rules/typing_any_used.md)

