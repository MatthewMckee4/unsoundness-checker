# Accessing `__defaults__` attribute

Detects accessing the `__defaults__` attribute of a function, which can bypass type checking and lead to runtime type errors.

The `__defaults__` attribute stores the default values for a function's parameters. When you modify this attribute directly, type checkers cannot verify that the new defaults match the expected parameter types, potentially causing type errors at runtime.

## What gets flagged

```python
def foo(x: int = 1) -> int:
    return x

foo.__defaults__ = ("string",)

foo.__defaults__ = None

def bar(x: int = 1, y: str = "string", z: bool = True) -> int:
    return x

bar.__defaults__ = (1, "string", "string")

bar.__defaults__ = None

bar.__defaults__ = (1, "string")

```

## What is okay

We do not emit errors if the types of the new defaults match the expected parameter types.

```python
def foo(x: str = "string") -> str:
    return x

foo.__defaults__ = ("another_string",)

foo.__defaults__ = ("another_string", "another_string")

def bar(x: int): ...

bar.__defaults__ = (1,)

def baz(x: int): ...

baz.__defaults__ = None
```
