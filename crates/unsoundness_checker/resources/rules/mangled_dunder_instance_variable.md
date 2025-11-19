# Mangled dunder instance variable used

Detects explicit usage of mangled dunder instance variables, which can bypass type checking and lead to runtime errors.

Python automatically mangles double-underscore (dunder) instance variables to `_ClassName__variable` to provide name privacy. When code explicitly uses the mangled form, it can assign different types to the mangled name than what the non-mangled variable expects, breaking type safety.

## What gets flagged

```python
class HiddenDunderVariables:
    def __init__(self, x: int) -> None:
        self.__str_x = str(x)
        self._HiddenDunderVariables__str_x = x

    def get_str_x(self) -> str:
        return self.__str_x


hidden_dunder_variables = HiddenDunderVariables(42)

# Type checker thinks this is a string, but at runtime it is an integer.
x = hidden_dunder_variables.get_str_x()

# We should also not be able to set it
hidden_dunder_variables._HiddenDunderVariables__str_x = 1

```

In this example, `self.__str_x` is first assigned a string value `str(x)`. However, the explicit use of the mangled name `self._HiddenDunderVariables__str_x` then assigns an integer `x` to the same underlying variable. This overwrites the string with an integer, causing the type checker to miss the error since it sees these as separate assignments.
