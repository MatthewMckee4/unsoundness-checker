# Categories

This page describes the different categories of type system unsoundness that the checker can detect.

## runtime-modification

Runtime code modifications that escape static type checker analysis.

### Rules in this category

- [`invalid-function-defaults`](rules.md#invalid-function-defaults) - detects invalid mutation of the `__defaults__` attribute of a function
- [`mutating-function-code-attribute`](rules.md#mutating-function-code-attribute) - detects mutating the `__code__` attribute of a function
- [`mutating-globals-dict`](rules.md#mutating-globals-dict) - detects mutations to the `globals()` dictionary
- [`invalid-setattr`](rules.md#invalid-setattr) - detects invalid usage of `setattr()` built-in function

## type-checking-suppression

Mechanisms that suppress or bypass type checker warnings.

### Rules in this category

- [`typing-any-used`](rules.md#typing-any-used) - detects usage of `typing.Any` in type annotations
- [`type-checking-directive-used`](rules.md#type-checking-directive-used) - detects usage of type checking directives in comments
- [`if-type-checking-used`](rules.md#if-type-checking-used) - detects usage of `if TYPE_CHECKING:` blocks
- [`typing-cast-used`](rules.md#typing-cast-used) - detects usage of `typing.cast()` function calls
- [`typing-type-is-used`](rules.md#typing-type-is-used) - detects usage of `typing.TypeIs` in return type annotations
- [`callable-ellipsis-used`](rules.md#callable-ellipsis-used) - detects usage of `...` in the first argument of `Callable` type annotations
- [`mangled-dunder-instance-variable`](rules.md#mangled-dunder-instance-variable) - detects explicit usage of mangled dunder instance variables
