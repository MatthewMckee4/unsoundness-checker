# Setting `__code__` attribute

Detects setting the `__code__` attribute of a function, which can bypass type checking and lead to runtime type errors.

The `__code__` attribute contains the compiled bytecode of a function. When you modify this attribute directly, you can completely change the function's behavior, including its parameter types and return type, without the type checker being aware of the change.

## What gets flagged

### Setting `__code__`

```python
def foo(x: int) -> int:
    return 1

def bar(x: str) -> str:
    return "!"

foo.__code__ = bar.__code__

# Type checker things `result` is an `int`, but it is a string
result = foo(10)
```
