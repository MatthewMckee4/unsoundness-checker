# Rules

## `invalid-function-defaults`

<small>
Default level: `error`.
</small>

**What it does**

Checks for invalid settings of the `__defaults__` attribute of a function.

**Why is this bad?**

Modifying the `__defaults__` attribute with types different to the parameters
can lead to runtime type errors.

**Examples**

```python
def foo(x: int = 1) -> int:
    return x

foo.__defaults__ = ("string",)
result = foo()  # Returns "string" but type checker thinks it's int
```

[See more](rules/invalid_function_defaults.md)

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

## `setting-function-code-attribute`

<small>
Default level: `error`.
</small>

**What it does**

Checks for setting the `__code__` attribute of a function.

**Why is this bad?**

Modifying the `__code__` attribute allows runtime modification
of function internals, which can bypass type checking and lead to runtime type errors.
Type checkers cannot analyze or verify operations performed through code objects.

**Examples**

```python
def foo(x: int) -> int:
    return 1

def bar(x: str) -> str:
    return "bar"

foo.__code__ = bar.__code__
# Now foo will return a string
```

[See more](rules/setting_function_code_attribute.md)

## `if-type-checking-used`

<small>
Default level: `warn`.
</small>

**What it does**

Checks for usage of `if TYPE_CHECKING:` blocks from the `typing` module.

**Why is this bad?**

`TYPE_CHECKING` is `False` at runtime but `True` during static type checking.
When used with an `else` clause where signatures don't match, the type checker
validates against the `if TYPE_CHECKING` branch, but at runtime the `else` branch
executes, causing runtime type errors that the type checker can't catch.

**Examples**

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    def get_value() -> int:
        ...
else:
    def get_value() -> str:
        return "hello"

# Type checker sees: get_value() -> int
# Runtime executes: get_value() -> str
result: int = get_value()  # Type checks, but returns str at runtime!
```

[See more](rules/if_type_checking_used.md)

## `type-checking-directive-used`

<small>
Default level: `warn`.
</small>

**What it does**

Checks for usage of type checking directives in comments.

**Why is this bad?**

Type checking directives like `# type: ignore` suppress type checker warnings,
which can hide potential type errors that may lead to runtime failures.
These directives bypass the safety guarantees that type checking provides.

**Examples**

```python
# mypy / standard (PEP 484)
x = "string" + 123  # type: ignore
y = foo()  # type: ignore[attr-defined]
```

[See more](rules/type_checking_directive_used.md)

## `typing-any-used`

<small>
Default level: `warn`.
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

## `typing-overload-used`

<small>
Default level: `warn`.
</small>

**What it does**

Checks for usage of overloaded functions.

**Why is this bad?**

Using overloaded functions can lead to runtime errors.
When users don't follow the correct overload implementation, it can lead to unexpected behavior.

**Examples**

```python
from typing import overload

@overload
def foo(x: int) -> str: ...
@overload
def foo(x: str) -> int: ...
def foo(x: int | str) -> int | str:
    return x
```

[See more](rules/typing_overload_used.md)

