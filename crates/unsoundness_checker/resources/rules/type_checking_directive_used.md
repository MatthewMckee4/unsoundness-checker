# Type checking directive used

Detects usage of type checking directives in comments, which suppress type checker warnings and may hide potential type errors.

Type checking directives like `# type: ignore` tell type checkers to skip validation of specific lines or sections of code. While sometimes necessary, they can mask real bugs that would otherwise be caught during static analysis.

## What gets flagged

We detect directives from the major Python type checkers:

### mypy / Standard (PEP 484)

```python
# Blanket ignore - suppresses all type errors on this line
result = unsafe_operation()  # type: ignore

# Specific error code - suppresses only specific error types
x = get_value()  # type: ignore[attr-defined]
y = compute()  # type: ignore[arg-type]
```

### pyright

```python
# Blanket ignore
data = risky_call()  # pyright: ignore

# Specific report type
value = process()  # pyright: ignore[reportOptionalMemberAccess]
item = fetch()  # pyright: ignore[reportGeneralTypeIssues]
```

### ty

```python
result = unsafe_operation()  # ty: ignore[unresolved-import]
data = get_data()  # ty: ignore[attr-defined]
```

### pyrefly

```python
value = process()  # pyrefly: ignore[unused-import]
item = fetch()  # pyrefly: ignore[type-error]
```

## Why this matters

Type checking directives bypass the safety guarantees that static type checking provides. For example:

```python
def calculate_total(items: list[int]) -> int:
    return sum(items)

# This will fail at runtime, but the directive hides the error
result = calculate_total("not a list")  # type: ignore
```

Instead of suppressing the error, fix the underlying type issue:

```python
def calculate_total(items: list[int]) -> int:
    return sum(items)

# Correct the type to match the function signature
result = calculate_total([1, 2, 3])
```

## When directives might be necessary

There are legitimate cases where type checking directives are needed:
- Working with dynamic third-party libraries that lack type stubs
- Handling complex metaprogramming that confuses type checkers
- Temporary workarounds during incremental type adoption

However, these should be rare and well-documented exceptions, not the norm.
