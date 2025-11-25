# Mutating `__code__` attribute - Extensive Tests

This file contains extensive tests for the mutating function code attribute rule, covering various scenarios where changing `__code__` bypasses type checking.

## What gets flagged

### Basic code swapping

```python
def foo(x: int) -> int:
    return 1

def bar(x: str) -> str:
    return "!"

foo.__code__ = bar.__code__
result = foo(10)
```

### Code swapping with different signatures

```python
def add(a: int, b: int) -> int:
    return a + b

def concat(s: str) -> str:
    return s + "!"

add.__code__ = concat.__code__
```

### Method code replacement

```python
class Calculator:
    def add(self, x: int, y: int) -> int:
        return x + y

    def multiply(self, x: str, y: str) -> str:
        return x + y

Calculator.add.__code__ = Calculator.multiply.__code__
```

### Lambda code replacement

```python
func1 = lambda x: x + 1
func2 = lambda x: str(x)

func1.__code__ = func2.__code__
```

### Nested function code swap

```python
def outer():
    def inner1(x: int) -> int:
        return x

    def inner2(x: str) -> str:
        return x

    inner1.__code__ = inner2.__code__
    return inner1
```

### Class method code swap

```python
class Handler:
    @classmethod
    def handle_int(cls, x: int) -> int:
        return x

    @classmethod
    def handle_str(cls, x: str) -> str:
        return x

Handler.handle_int.__code__ = Handler.handle_str.__code__
```

### Static method code swap

```python
class Utils:
    @staticmethod
    def process_int(x: int) -> int:
        return x

    @staticmethod
    def process_str(x: str) -> str:
        return x

Utils.process_int.__code__ = Utils.process_str.__code__
```

### Property code swap

```python
class Container:
    @property
    def value(self) -> int:
        return 0

def wrong_getter(self) -> str:
    return "wrong"

Container.value.fget.__code__ = wrong_getter.__code__
```

### Swapping with builtin-like function

```python
def custom_len(obj: list) -> int:
    return 0

def custom_str(obj: list) -> str:
    return ""

custom_len.__code__ = custom_str.__code__
```

### Code swap in loop

```python
def func1(x: int) -> int:
    return x

def func2(x: str) -> str:
    return x

for _ in range(5):
    func1.__code__ = func2.__code__
```

### Conditional code swap

```python
def option_a(x: int) -> int:
    return x

def option_b(x: str) -> str:
    return x

if True:
    option_a.__code__ = option_b.__code__
```

### Multiple code swaps

```python
def f1(x: int) -> int:
    return x

def f2(x: str) -> str:
    return x

def f3(x: bool) -> bool:
    return x

f1.__code__ = f2.__code__
f2.__code__ = f3.__code__
```

### Code swap with different param count

```python
def one_param(x: int) -> int:
    return x

def two_params(x: int, y: int) -> int:
    return x + y

one_param.__code__ = two_params.__code__
```

### Code swap in class init

```python
class Swapper:
    def __init__(self) -> None:
        def func1(x: int) -> int:
            return x

        def func2(x: str) -> str:
            return x

        func1.__code__ = func2.__code__
        self.func = func1
```

### Code swap with decorator

```python
def decorator(func):
    def wrapper(x: int) -> int:
        return func(x)
    return wrapper

def wrong(x: str) -> str:
    return x

@decorator
def target(x: int) -> int:
    return x

target.__code__ = wrong.__code__
```

## What we can't catch

### Type-safe code swap

```python
def foo(x: int) -> int:
    return 1

def bar(x: int) -> int:
    return 1

foo.__code__ = bar.__code__
result = foo(10)
```
