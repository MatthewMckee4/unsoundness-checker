# Type checking directive used - Extensive Tests

This file contains extensive tests for the type checking directive rule, covering various type checker ignore directives.

## What gets flagged

### mypy type ignore

```python
res = foo()  # type: ignore
```

### mypy with specific error code

```python
res = foo()  # type: ignore[name-defined]
```

### Multiple mypy error codes

```python
res = foo()  # type: ignore[name-defined, arg-type]
```

### pyright ignore

```python
res = foo()  # pyright: ignore
```

### pyright with error code

```python
res = foo()  # pyright: ignore[reportUndefinedVariable]
```

### ty ignore

```python
res = foo()  # ty: ignore
```

### ty with error code

```python
res = foo()  # ty: ignore[unresolved-reference]
```

### pyrefly ignore

```python
res = foo()  # pyrefly: ignore
```

### pyrefly with error code

```python
res = foo()  # pyrefly: ignore[unknown-name]
```

### Inline type ignore on assignment

```python
x: int = "wrong"  # type: ignore
```

### Type ignore on function call

```python
def process(x: int) -> int:
    return x

result = process("wrong")  # type: ignore
```

### Type ignore on method call

```python
class Handler:
    def handle(self, x: int) -> None:
        pass

h = Handler()
h.handle("wrong")  # type: ignore
```

### Type ignore on import

```python
from nonexistent import something  # type: ignore
```

### Type ignore on attribute access

```python
obj = get_object()
value = obj.nonexistent  # type: ignore
```

### Type ignore in class

```python
class MyClass:
    value: int = "wrong"  # type: ignore
```

### Type ignore on return

```python
def get_number() -> int:
    return "not_int"  # type: ignore
```

### Multiple directives same line

```python
x = foo()  # type: ignore  # pyright: ignore
```

### Type ignore with other comments

```python
x = calculate()  # type: ignore - TODO: fix this later
```

### Type ignore in list comprehension

```python
values = [foo(x) for x in items]  # type: ignore
```

### Type ignore in dict comprehension

```python
mapping = {k: wrong(v) for k, v in pairs}  # type: ignore
```

### Type ignore on generator

```python
gen = (process(x) for x in items)  # type: ignore
```

### Type ignore with cast

```python
from typing import cast
value = cast(int, "string")  # type: ignore
```

### Type ignore on decorator

```python
@some_decorator  # type: ignore
def my_function():
    pass
```

### Type ignore on with statement

```python
with open("file") as f:  # type: ignore
    pass
```

### Type ignore on context manager

```python
with wrong_context_manager():  # type: ignore
    pass
```

### Type ignore on assert

```python
assert wrong_condition()  # type: ignore
```

### Type ignore on raise

```python
raise WrongException()  # type: ignore
```

### Type ignore on try-except

```python
try:
    risky_operation()  # type: ignore
except Exception:
    pass
```

### Type ignore in f-string

```python
message = f"Value: {wrong_func()}"  # type: ignore
```

### Ignore on binary operation

```python
result = "string" + 123  # type: ignore
```

### Ignore on comparison

```python
if "string" > 123:  # type: ignore
    pass
```

## Why this matters

### Hidden runtime error

```python
def calculate_total(items: list[int]) -> int:
    return sum(items)

result = calculate_total("not a list")  # type: ignore
```

### Masking attribute error

```python
class Person:
    name: str

p = Person()
age = p.age  # type: ignore
```

### Hiding incompatible return

```python
def get_number() -> int:
    return "not a number"  # type: ignore
```

## Ensure we don't catch examples of "type: ignore" randomly in a comment

```python
class Foo:
    # 'type: ignore'
    # foo type: ignore
    # foo 'type: ignore'
