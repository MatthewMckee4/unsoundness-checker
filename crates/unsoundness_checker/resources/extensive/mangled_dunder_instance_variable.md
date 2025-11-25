# Mangled dunder instance variable used - Extensive Tests

This file contains extensive tests for the mangled dunder instance variable rule, covering various scenarios where explicit mangling bypasses type checking.

## What gets flagged

### Basic mangled variable assignment

```python
class HiddenDunderVariables:
    def __init__(self, x: int) -> None:
        self.__str_x = str(x)
        self._HiddenDunderVariables__str_x = x

    def get_str_x(self) -> str:
        return self.__str_x

hidden_dunder_variables = HiddenDunderVariables(42)
x = hidden_dunder_variables.get_str_x()
```

### External mangled variable access

```python
class HiddenDunderVariables:
    def __init__(self, x: int) -> None:
        self.__str_x = str(x)

hidden_dunder_variables = HiddenDunderVariables(42)
hidden_dunder_variables._HiddenDunderVariables__str_x = 1
```

### Multiple mangled variables

```python
class Container:
    def __init__(self) -> None:
        self.__data: str = "hello"
        self.__count: int = 0

obj = Container()
obj._Container__data = 123
obj._Container__count = "not_int"
```

### Mangled in method

```python
class Processor:
    def __init__(self) -> None:
        self.__value: int = 0

    def update(self) -> None:
        self._Processor__value = "wrong_type"
```

### Reading mangled variable

```python
class Secret:
    def __init__(self) -> None:
        self.__secret: str = "password"

obj = Secret()
obj._Secret__secret = 123
value = obj._Secret__secret
```

### Mangled in property

```python
class Managed:
    def __init__(self) -> None:
        self.__data: int = 0

    @property
    def data(self) -> int:
        return self._Managed__data

obj = Managed()
obj._Managed__data = "not_int"
```

### Mangled in inheritance

```python
class Parent:
    def __init__(self) -> None:
        self.__parent_var: str = "parent"

class Child(Parent):
    def __init__(self) -> None:
        super().__init__()
        self.__child_var: int = 0

child = Child()
child._Parent__parent_var = 123
child._Child__child_var = "wrong"
```

### Mangled with classmethod

```python
class Factory:
    __counter: int = 0

    @classmethod
    def increment(cls) -> None:
        cls._Factory__counter += 1

Factory._Factory__counter = "not_int"
```

### Nested class mangled

```python
class Outer:
    class Inner:
        def __init__(self) -> None:
            self.__value: int = 0

inner = Outer.Inner()
inner._Inner__value = "wrong"
```

### Mangled in __init__ vs method

```python
class State:
    def __init__(self) -> None:
        self.__state: str = "init"

    def update(self) -> None:
        self._State__state = 123
```

### Multiple classes same mangled name

```python
class First:
    def __init__(self) -> None:
        self.__data: int = 0

class Second:
    def __init__(self) -> None:
        self.__data: str = "hello"

f = First()
s = Second()
f._First__data = "wrong"
s._Second__data = 999
```

### Mangled in lambda

```python
class Container:
    def __init__(self) -> None:
        self.__value: int = 0
        self.updater = lambda: setattr(self, "_Container__value", "wrong")
```

### Mangled with del

```python
class Temporary:
    def __init__(self) -> None:
        self.__temp: str = "temp"

obj = Temporary()
obj._Temporary__temp = 123
```

### Mangled in class variable

```python
class Config:
    __setting: int = 0

Config._Config__setting = "not_int"
```

### Complex mangled access

```python
class Complex:
    def __init__(self) -> None:
        self.__x: int = 0
        self.__y: str = ""

obj = Complex()
values = {"x": "wrong", "y": 123}
obj._Complex__x = values["x"]
obj._Complex__y = values["y"]
```
