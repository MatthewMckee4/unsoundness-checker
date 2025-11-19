# Mutable default argument in generic function

Detects when a generic function or method accepts a mutable object as a default parameter value, which can lead to runtime type errors.

When a generic function uses a mutable default value (like a list, dict, or set), that default is shared across all invocations of the function. This creates a scenario where the mutable object can accumulate values of different types from different calls.

## What gets flagged

```python
def append_and_return[T](x: T, y: list[T] = []) -> list[T]:
    y.append(x)
    return y

# Each call with a different type shares the same list
int_list = append_and_return(1)
str_list = append_and_return("hello")

# This is a int at runtime but str at type check time.
value = str_list[0]
```

In this example, both calls to `append_and_return` share the same default list. If code appends to this list in different calls with different type parameters, the list ends up containing mixed types, breaking the type soundness that `list[T]` promises.
