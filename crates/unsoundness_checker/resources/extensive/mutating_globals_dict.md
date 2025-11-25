# Mutating `globals()` dict - Extensive Tests

This file contains extensive tests for the mutating globals dict rule, covering various scenarios where globals() mutations bypass type checking.

## What gets flagged

### Basic wrong type assignment

```python
x: int = 5

globals()['x'] = "hello"

result: int = x
```

### Multiple global mutations

```python
a: int = 1
b: str = "hello"
c: bool = True

globals()['a'] = "wrong"
globals()['b'] = 123
globals()['c'] = "not_bool"
```

### Mutation in function

```python
count: int = 0

def increment() -> None:
    globals()['count'] = "not_int"
```

### Mutation with computed key

```python
value: int = 42

key = 'value'
globals()[key] = "wrong_type"
```

### Creating new globals

```python
globals()['new_var'] = "hello"
globals()['another'] = 123
```

### Mutation in class method

```python
total: int = 0

class Counter:
    def update(self) -> None:
        globals()['total'] = "not_int"
```

### Mutation in loop

```python
data: list[int] = []

for i in range(5):
    globals()['data'] = "wrong"
```

### Conditional mutation

```python
flag: bool = True

if flag:
    globals()['flag'] = "not_bool"
```

### Nested function mutation

```python
outer_var: int = 0

def outer():
    def inner():
        globals()['outer_var'] = "wrong"
    inner()
```

### Mutation with del

```python
temp: str = "temp"

del globals()['temp']
```

### Update multiple at once

```python
x: int = 1
y: str = "hello"

globals().update({'x': "wrong", 'y': 123})
```

### Mutation with setdefault

```python
setting: int = 0

globals().setdefault('setting', "wrong_default")
```

### Mutation with pop

```python
value: int = 42

popped = globals().pop('value', "default")
```

### Clear globals (dangerous!)

```python
x: int = 1

# This would break everything but demonstrates the pattern
# globals().clear()
```

### Mutation in list comprehension

```python
items: list[int] = [1, 2, 3]

[globals().__setitem__('items', "wrong") for _ in range(1)]
```

### Mutation with __setitem__

```python
data: dict[str, int] = {}

globals().__setitem__('data', "not_dict")
```

## What is okay

### Type-compatible mutation

```python
x: int = 1

globals()["x"] = 2
```

### Assigning subtype

```python
from typing import Sequence

seq: Sequence[int] = [1, 2, 3]

globals()["seq"] = [4, 5, 6]
```

### Creating new untyped global

```python
globals()['dynamic'] = "anything"
```
