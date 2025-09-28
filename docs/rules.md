# Rules

## `typing-any-used`

<small>
Default level: [`error`](../rules.md#rule-levels "This lint has a default level of 'error'.") ·
</small>

**What it does**

Checks for usage of `typing.Any` in type annotations.

**Why is this bad?**

Using `typing.Any` in type annotations can lead to runtime errors.

**Examples**

```python
def foo(x: Any) -> Any:
    return x + 1

foo("1")
```

