# Evaluation

There are several ways we evaluate the unsoundness checker.

## Benchmark

Run the following to see the performance compared against real world projects:

```bash
cargo run --release -p evaluation benchmark
```

## Unsoundness Suite

Run the following to see the unsoundness suite:

```bash
cargo run --release -p evaluation unsoundness-suite
```

Below we discuss the following results of the unsoundness suite:

This currently fails.

```rs
UnsoundnessSuiteFile {
    path: "examples/stdlib/setattr.py",
    expected_diagnostics: DiagnosticId::Lint(rules::INVALID_SETATTR.name()),
},
```

This is due to the inferred type of an instance variable.
In the code in this file we have the following line:

```python
class ToString:
    def __init__(self, x: int) -> None:
      self.str_x = str(x)

foo = ToString(1)
setattr(foo, "str_x", 1)
```

Here `ty` infers `Unknown | str` for `self.str_x`.
This means when we make the call to setattr str_x to an integer it is okay because `int` is assignable to `Unknown`.

This currently fails.

```rs
UnsoundnessSuiteFile {
    path: "examples/runtime/globals.py",
    expected_diagnostics: DiagnosticId::Lint(rules::MUTATING_GLOBALS_DICT.name()),
},
```

Here is the code from the file:

```py
def func(x: int) -> str:
    return str(x)

globals()["func"] = lambda x: x
```

This is "okay" (we don't flag it) because the inferred type of the lambda function is `(Unknown) -> Unknown`.
This type is assignable to `(int) -> str`, so we do not catch an error here.
