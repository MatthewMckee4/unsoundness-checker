# Categories

This page describes the different categories of type system unsoundness that the checker can detect.

## runtime-modification

Runtime code modifications that escape static type checker analysis.

### Rules in this category

- [`invalid-function-defaults`](rules.md#invalid-function-defaults) - detects invalid setting of the `__defaults__` attribute of a function
- [`setting-function-code-attribute`](rules.md#setting-function-code-attribute) - detects setting the `__code__` attribute of a function

## type-checking-suppression

Mechanisms that suppress or bypass type checker warnings.

### Rules in this category

- [`typing-any-used`](rules.md#typing-any-used) - detects usage of `typing.Any` in type annotations
- [`type-checking-directive-used`](rules.md#type-checking-directive-used) - detects usage of type checking directives in comments
- [`if-type-checking-used`](rules.md#if-type-checking-used) - detects usage of `if TYPE_CHECKING:` blocks
