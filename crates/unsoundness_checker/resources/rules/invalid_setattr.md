# Invalid `setattr()` call

Detects invalid usage of `setattr()` built-in function, which bypasses type checking and can lead to runtime type errors.

`setattr()` allows dynamic attribute assignment, letting you assign any type to any attribute regardless of type annotations. Type checkers can't track these dynamic modifications.

## What gets flagged

```py
class Foo:
    def __init__(self) -> None:
        self.x: str = "hello"

foo = Foo()
setattr(foo, "x", 1)
```
