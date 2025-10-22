# Setting `__code__` attribute

Detects setting the `__code__` attribute of a function, which can bypass type checking and lead to runtime type errors.

The `__code__` attribute contains the compiled bytecode of a function. When you modify this attribute directly, you can completely change the function's behavior, including its parameter types and return type, without the type checker being aware of the change.

## What gets flagged

### Setting `__code__`

```python
def foo(x: int) -> int:
    return x + 1

def bar(x: str) -> str:
    return x + "!"

# Setting __code__ - this completely bypasses type checking
foo.__code__ = bar.__code__

# Now foo has the wrong signature
result: int = foo(10)  # TypeError at runtime: can only concatenate str to str
```

## Why this matters

When you replace a function's `__code__` attribute, you can make the function do something completely different from what its type signature indicates:

```python
def calculate(amount: int) -> int:
    return amount * 2

def greet(name: str) -> str:
    return f"Hello, {name}!"

# Type checker doesn't catch this
calculate.__code__ = greet.__code__

# This will crash at runtime
result = calculate(100)  # TypeError: can't multiply sequence by non-int
```

Type checkers cannot track these mutations, so code that appears type-safe can still fail at runtime.

## Best practices

- Don't modify `__code__` directly
- If you need to change function behavior, create a new function
- Use decorators or function factories for dynamic behavior
- Consider using functools.wraps if you need to preserve function metadata
