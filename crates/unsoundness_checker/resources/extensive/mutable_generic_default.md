# Mutable default argument in generic function - Extensive Tests

This file contains extensive tests for the mutable generic default rule, covering various scenarios where mutable defaults can cause type unsoundness.

## What gets flagged

### Basic mutable list default

```python
def append_and_return[T](x: T, y: list[T] = []) -> list[T]:
    y.append(x)
    return y

int_list = append_and_return(1)
str_list = append_and_return("hello")
value = str_list[0]
```

### Mutable dict default

```python
def add_to_dict[K, V](key: K, value: V, mapping: dict[K, V] = {}) -> dict[K, V]:
    mapping[key] = value
    return mapping

int_dict = add_to_dict(1, "one")
str_dict = add_to_dict("key", 42)
```

### Mutable set default

```python
def add_to_set[T](item: T, items: set[T] = set()) -> set[T]:
    items.add(item)
    return items

int_set = add_to_set(1)
str_set = add_to_set("hello")
```

### Multiple mutable defaults

```python
def multi_defaults[T](x: T, lst: list[T] = [], dct: dict[str, T] = {}) -> tuple[list[T], dict[str, T]]:
    lst.append(x)
    dct["key"] = x
    return lst, dct
```

### Nested generic with mutable default

```python
def nested_generic[T](items: T, container: list[list[T]] = [[]]) -> list[list[T]]:
    container[0].append(items)
    return container
```

### Method with mutable default

```python
class Container[T]:
    def add_items(self, item: T, items: list[T] = []) -> list[T]:
        items.append(item)
        return items
```

### Classmethod with mutable default

```python
class Factory[T]:
    @classmethod
    def create(cls, value: T, cache: dict[str, T] = {}) -> dict[str, T]:
        cache["last"] = value
        return cache
```

### Staticmethod with mutable default

```python
class Processor[T]:
    @staticmethod
    def process(item: T, history: list[T] = []) -> list[T]:
        history.append(item)
        return history
```

### Multiple type parameters with mutable default

```python
def pair_store[K, V](key: K, value: V, pairs: list[tuple[K, V]] = []) -> list[tuple[K, V]]:
    pairs.append((key, value))
    return pairs
```

### Generic with bound and mutable default

```python
def numeric_accumulator[T: (int, float)](x: T, values: list[T] = []) -> list[T]:
    values.append(x)
    return values
```

### Mutable default with complex generic type

```python
from typing import TypeVar

T = TypeVar('T')

def complex_default(item: T, store: dict[type, list[T]] = {}) -> dict[type, list[T]]:
    item_type = type(item)
    if item_type not in store:
        store[item_type] = []
    store[item_type].append(item)
    return store
```

### Lambda with mutable default (if generic)

```python
make_accumulator = lambda x, lst=[]: (lst.append(x), lst)[1]
```

### Nested function with mutable default

```python
def outer[T](value: T):
    def inner(item: T, items: list[T] = []) -> list[T]:
        items.append(item)
        return items
    return inner(value)
```

### Mutable bytearray default

```python
def byte_accumulator[T](data: T, buffer: bytearray = bytearray()) -> bytearray:
    buffer.extend(str(data).encode())
    return buffer
```

### Multiple generics with shared mutable

```python
def shared_store[T, U](t_val: T, u_val: U, store: list[T | U] = []) -> list[T | U]:
    store.append(t_val)
    store.append(u_val)
    return store
```
