# Mutating `globals()` dict

Detects mutations to the `globals()` dictionary, which can bypass type checking and lead to runtime type errors.

The `globals()` function returns a dictionary representing the current global symbol table. When you modify this dictionary directly, you can change the types of global variables at runtime without the type checker being aware of the change.

## What gets flagged

```py
x: int = 5

globals()['x'] = "hello"

# Type checker thinks `result` is an `int`, but it is a string
result: int = x
```

## What is okay

If the type of the new value is assignable to the existing type of the symbol, we allow the mutation.

```py
x: int = 1

# `Literal[2]` is assignable to `int`
globals()["x"] = 2
```
