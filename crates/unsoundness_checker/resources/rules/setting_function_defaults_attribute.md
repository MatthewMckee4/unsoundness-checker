# Accessing `__defaults__` attribute

Detects accessing the `__defaults__` attribute of a function, which can bypass type checking and lead to runtime type errors.

The `__defaults__` attribute stores the default values for a function's parameters. When you modify this attribute directly, type checkers cannot verify that the new defaults match the expected parameter types, potentially causing type errors at runtime.

## What gets flagged

### Reading `__defaults__`

```python
def foo(x: int = 1) -> int:
    return x

# Accessing the defaults
defaults = foo.__defaults__
```

### Writing to `__defaults__`

```python
def foo(x: int = 1) -> int:
    return x

# Modifying defaults - this can bypass type checking
foo.__defaults__ = ("string",)  # Type checker thinks x is int, but it's actually str
result = foo()  # Returns "string" but type is int
```

## Why this matters

When you mutate `__defaults__` directly, you can assign values that don't match the parameter types:

```python
def calculate(amount: int = 10) -> int:
    return amount * 2

# Type checker doesn't catch this
calculate.__defaults__ = ("not a number",)

# This will crash at runtime
result = calculate()  # TypeError: can't multiply sequence by non-int
```

Type checkers cannot track these mutations, so code that appears type-safe can still fail at runtime.

## Best practices

- Don't access or modify `__defaults__` directly
- Use proper function calls with explicit arguments instead
- If you need to change default behavior, create a new function or use functools.partial
