# Type checking directive used

Detects usage of type checking directives in comments, which suppress type checker warnings and may hide potential type errors.

Type checking directives like `# type: ignore` tell type checkers to skip validation of specific lines or sections of code. While sometimes necessary, they can mask real bugs that would otherwise be caught during static analysis.

## What gets flagged

We detect directives from the major Python type checkers:

### mypy / Standard (PEP 484)

```python
res = foo()  # type: ignore

res = foo()  # type: ignore[name-defined]
```

### pyright

```python
res = foo()  # pyright: ignore

res = foo()  # pyright: ignore[reportUndefinedVariable]
```

### ty

```python
res = foo()  # ty: ignore

res = foo()  # ty: ignore[unresolved-reference]
```

### pyrefly

```python
res = foo()  # pyrefly: ignore

res = foo()  # pyrefly: ignore[unknown-name]
```

## Why this matters

Type checking directives bypass the safety guarantees that static type checking provides. For example:

```python
def calculate_total(items: list[int]) -> int:
    return sum(items)

# This will fail at runtime, but the directive hides the error
result = calculate_total("not a list")  # type: ignore
```

## When directives might be necessary

There are legitimate cases where type checking directives are needed:
- Working with dynamic third-party libraries that lack type stubs
- Temporary workarounds during incremental type adoption

However, these should be rare and well-documented exceptions, not the norm.
