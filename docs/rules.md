# Rules

## `invalid-function-defaults`


**What it does**

Checks for invalid mutations of the `__defaults__` attribute of a function.

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

<small>
Default level: `error`.
</small>

<small>
Categories: [`runtime-modification`](categories.md#runtime-modification).
</small>

[See more](rules/invalid_function_defaults.md)

## `invalid-overload-implementation`


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

<small>
Default level: `error`.
</small>

<small>
Categories: None.
</small>

[See more](rules/invalid_overload_implementation.md)

## `mutable-generic-default`


**What it does**

Checks for generic functions or methods that accept mutable objects as default parameter values.

**Why is this bad?**

When a generic function uses a mutable default value (like a list, dict, or set), that default
is shared across all invocations of the function. This creates a scenario where the mutable
object can accumulate values of different types from different calls.

Type checkers assume that `list[T]` only contains values of type `T`. However, when a mutable
default is reused across calls with different type parameters, the list can contain values of
multiple different types, leading to runtime type errors.

**Examples**

```python
def append_and_return[T](x: T, items: list[T] = []) -> list[T]:
    items.append(x)
    return items

int_list = append_and_return(42)
str_list = append_and_return("hello")

# This is a int at runtime but str at type check time.
value: str = str_list[0]
```

<small>
Default level: `error`.
</small>

<small>
Categories: None.
</small>

[See more](rules/mutable_generic_default.md)

## `mutating-function-code-attribute`


**What it does**

Checks for mutating the `__code__` attribute of a function.

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

<small>
Default level: `error`.
</small>

<small>
Categories: [`runtime-modification`](categories.md#runtime-modification).
</small>

[See more](rules/mutating_function_code_attribute.md)

## `mutating-globals-dict`


**What it does**

Checks for mutations to the `globals()` dictionary.

**Why is this bad?**

Modifying the `globals()` dictionary allows runtime modification
of global variables, which can bypass type checking and lead to runtime type errors.
Type checkers cannot track or verify modifications made through the globals dictionary.

**Examples**

```python
x: int = 5

globals()['x'] = "hello"

# Type checker thinks `x` is an `int`, but it is now a string
result: int = x
```

<small>
Default level: `error`.
</small>

<small>
Categories: [`runtime-modification`](categories.md#runtime-modification).
</small>

[See more](rules/mutating_globals_dict.md)

## `callable-ellipsis-used`


**What it does**

Checks for usage of `...` (ellipsis) in the first argument of `Callable` type annotations.

**Why is this bad?**

Using `Callable[..., ReturnType]` indicates that the callable accepts any number
of arguments of any type, which bypasses parameter type checking. This can lead to
runtime type errors as the type checker cannot verify argument types or counts.

**Examples**

```python
from typing import Callable

def foo(callback: Callable[..., int]) -> int:
    return callback("wrong", "types")

def bar(x: int) -> int:
    return x

# This passes type checking but fails at runtime.
foo(bar)
```

<small>
Default level: `warn`.
</small>

<small>
Categories: [`type-checking-suppression`](categories.md#type-checking-suppression).
</small>

[See more](rules/callable_ellipsis_used.md)

## `if-type-checking-used`


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

result: int = get_value()  # Type checks, but returns str at runtime!
```

<small>
Default level: `warn`.
</small>

<small>
Categories: [`type-checking-suppression`](categories.md#type-checking-suppression).
</small>

[See more](rules/if_type_checking_used.md)

## `mangled-dunder-instance-variable`


**What it does**

Checks for explicit usage of mangled dunder instance variables in attribute access.

**Why is this bad?**

Python automatically mangles double-underscore (dunder) instance variables to
`_ClassName__variable` to provide name privacy. When code explicitly uses the
mangled form, it can bypass type checking by assigning different types to the
mangled name than what the non-mangled variable expects.

**Examples**

```python
class HiddenDunderVariables:
    def __init__(self, x: int) -> None:
        self.__str_x = str(x)
        self._HiddenDunderVariables__str_x = x

    def get_str_x(self) -> str:
        return self.__str_x

# Here, x is a string at type check time, but an integer at runtime.
x = hidden_dunder_variables.get_str_x()
```

<small>
Default level: `warn`.
</small>

<small>
Categories: None.
</small>

[See more](rules/mangled_dunder_instance_variable.md)

## `type-checking-directive-used`


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

<small>
Default level: `warn`.
</small>

<small>
Categories: [`type-checking-suppression`](categories.md#type-checking-suppression).
</small>

[See more](rules/type_checking_directive_used.md)

## `typing-any-used`


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

<small>
Default level: `warn`.
</small>

<small>
Categories: [`type-checking-suppression`](categories.md#type-checking-suppression).
</small>

[See more](rules/typing_any_used.md)

## `typing-cast-used`


**What it does**

Checks for usage of `typing.cast()` function calls.

**Why is this bad?**

`typing.cast()` tells the type checker to treat a value as a specific type
without any runtime checks or validation. This can lead to runtime type errors
if the cast is incorrect. Type checkers trust casts completely, so incorrect
casts bypass all type safety guarantees.

**Examples**

```python
from typing import cast

def get_value() -> int | str:
    return "hello"

result = cast(int, get_value())
result + 1  # Type checks, but fails at runtime!
```

<small>
Default level: `warn`.
</small>

<small>
Categories: [`type-checking-suppression`](categories.md#type-checking-suppression).
</small>

[See more](rules/typing_cast_used.md)

## `typing-overload-used`


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

<small>
Default level: `warn`.
</small>

<small>
Categories: None.
</small>

[See more](rules/typing_overload_used.md)

## `typing-type-is-used`


**What it does**

Checks for return types that use `typing.TypeIs`.

**Why is this bad?**

Using `typing.TypeIs` in return type annotations can lead to runtime type errors.
Type checkers use `TypeIs` to narrow types, but incorrect implementation can bypass
type safety guarantees.

**Examples**

```python
from typing import TypeIs

def is_int(x: object) -> TypeIs[int]:
    return True

value = "hello"

if is_int(value):
    result = value + 1  # Type checks but fails at runtime!
```

<small>
Default level: `warn`.
</small>

<small>
Categories: [`type-checking-suppression`](categories.md#type-checking-suppression).
</small>

[See more](rules/typing_type_is_used.md)

