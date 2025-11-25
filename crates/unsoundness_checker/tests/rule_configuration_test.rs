pub mod common;

use common::TestRunner;

#[test]
fn test_typing_any_used_rule_enabled() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "error"
"#,
    );

    runner.add_file(
        "test.py",
        r"
from typing import Any

def foo(x: Any) -> Any:
    return x + 1
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:12
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |            ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:20
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |                    ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file
    ");
}

#[test]
fn test_typing_any_used_rule_disabled() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "ignore"
"#,
    );

    runner.add_file(
        "test.py",
        "
from typing import Any

def foo(x: Any) -> Any:
    return x + 1
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_invalid_overload_implementation_rule_enabled() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
invalid-overload-implementation = "error"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import overload

@overload
def foo(x: int) -> str: ...

@overload
def foo(x: str) -> int: ...

def foo(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r#"
    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
     --> test.py:5:5
      |
    4 | @overload
    5 | def foo(x: int) -> str: ...
      |     ^^^
    6 |
    7 | @overload
      |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def foo(x: str) -> int: ...
       |     ^^^
     9 |
    10 | def foo(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default

    error[invalid-overload-implementation]: Invalid overload implementation can lead to runtime errors.
      --> test.py:11:5
       |
    10 | def foo(x: int | str) -> int | str:
    11 |     return b""
       |     ^^^^^^^^^^
       |
    info: This overload implementation is invalid as `Literal[b""]` is not assignable to any of the overload return types (`str`, `int`)
    info: rule `invalid-overload-implementation` was selected in the configuration file
    "#);
}

#[test]
fn test_invalid_overload_implementation_rule_disabled() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
invalid-overload-implementation = "ignore"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import overload

@overload
def foo(x: int) -> str: ...

@overload
def foo(x: str) -> int: ...

def foo(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
     --> test.py:5:5
      |
    4 | @overload
    5 | def foo(x: int) -> str: ...
      |     ^^^
    6 |
    7 | @overload
      |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def foo(x: str) -> int: ...
       |     ^^^
     9 |
    10 | def foo(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default
    ");
}

#[test]
fn test_all_rules_enabled() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "error"
invalid-overload-implementation = "error"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import Any, overload

def any_func(x: Any) -> Any:
    return x + 1

@overload
def overload_func(x: int) -> str: ...

@overload
def overload_func(x: str) -> int: ...

def overload_func(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r#"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:17
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                 ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:25
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                         ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def overload_func(x: int) -> str: ...
       |     ^^^^^^^^^^^^^
     9 |
    10 | @overload
       |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:11:5
       |
    10 | @overload
    11 | def overload_func(x: str) -> int: ...
       |     ^^^^^^^^^^^^^
    12 |
    13 | def overload_func(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default

    error[invalid-overload-implementation]: Invalid overload implementation can lead to runtime errors.
      --> test.py:14:5
       |
    13 | def overload_func(x: int | str) -> int | str:
    14 |     return b""
       |     ^^^^^^^^^^
       |
    info: This overload implementation is invalid as `Literal[b""]` is not assignable to any of the overload return types (`str`, `int`)
    info: rule `invalid-overload-implementation` was selected in the configuration file
    "#);
}

#[test]
fn test_all_rules_disabled() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "ignore"
invalid-overload-implementation = "ignore"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import Any, overload

def any_func(x: Any) -> Any:
    return x + 1

@overload
def overload_func(x: int) -> str: ...

@overload
def overload_func(x: str) -> int: ...

def overload_func(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def overload_func(x: int) -> str: ...
       |     ^^^^^^^^^^^^^
     9 |
    10 | @overload
       |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:11:5
       |
    10 | @overload
    11 | def overload_func(x: str) -> int: ...
       |     ^^^^^^^^^^^^^
    12 |
    13 | def overload_func(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default
    ");
}

#[test]
fn test_mixed_rule_configuration() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "error"
invalid-overload-implementation = "ignore"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import Any, overload

def any_func(x: Any) -> Any:
    return x + 1

@overload
def overload_func(x: int) -> str: ...

@overload
def overload_func(x: str) -> int: ...

def overload_func(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:17
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                 ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:25
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                         ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def overload_func(x: int) -> str: ...
       |     ^^^^^^^^^^^^^
     9 |
    10 | @overload
       |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:11:5
       |
    10 | @overload
    11 | def overload_func(x: str) -> int: ...
       |     ^^^^^^^^^^^^^
    12 |
    13 | def overload_func(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default
    ");
}

#[test]
fn test_rule_warning_level() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "warn"
"#,
    );

    runner.add_file(
        "test.py",
        r"
from typing import Any

def foo(x: Any) -> Any:
    return x + 1
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:12
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |            ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:20
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |                    ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file
    ");
}

#[test]
fn test_default_rule_behavior() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "test.py",
        r"
from typing import Any, overload

def any_func(x: Any) -> Any:
    return x + 1

@overload
def overload_func(x: int) -> str: ...

@overload
def overload_func(x: str) -> int: ...

def overload_func(x: int | str) -> int | str:
    return x
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:17
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                 ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` is enabled by default

    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:25
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                         ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def overload_func(x: int) -> str: ...
       |     ^^^^^^^^^^^^^
     9 |
    10 | @overload
       |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:11:5
       |
    10 | @overload
    11 | def overload_func(x: str) -> int: ...
       |     ^^^^^^^^^^^^^
    12 |
    13 | def overload_func(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default
    ");
}

#[test]
fn test_typing_any_used_error_level() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "error"
"#,
    );

    runner.add_file(
        "test.py",
        r"
from typing import Any

def foo(x: Any) -> Any:
    return x + 1
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:12
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |            ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:20
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |                    ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file
    ");
}

#[test]
fn test_invalid_overload_implementation_error_level() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
invalid-overload-implementation = "error"
"#,
    );

    runner.add_file(
        "test.py",
        r"
from typing import overload

@overload
def foo(x: int) -> str: ...

@overload
def foo(x: str) -> int: ...

def foo(x: int | str) -> int | str:
    return x
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
     --> test.py:5:5
      |
    4 | @overload
    5 | def foo(x: int) -> str: ...
      |     ^^^
    6 |
    7 | @overload
      |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def foo(x: str) -> int: ...
       |     ^^^
     9 |
    10 | def foo(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default
    ");
}

#[test]
fn test_typing_any_used_warn_level() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "warn"
"#,
    );

    runner.add_file(
        "test.py",
        r"
from typing import Any

def foo(x: Any) -> Any:
    return x + 1
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:12
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |            ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:20
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> Any:
      |                    ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file
    ");
}

#[test]
fn test_invalid_overload_implementation_warn_level() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
invalid-overload-implementation = "warn"
"#,
    );

    runner.add_file(
        "test.py",
        r"
from typing import overload

@overload
def foo(x: int) -> str: ...

@overload
def foo(x: str) -> int: ...

def foo(x: int | str) -> int | str:
    return x
",
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r"
    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
     --> test.py:5:5
      |
    4 | @overload
    5 | def foo(x: int) -> str: ...
      |     ^^^
    6 |
    7 | @overload
      |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def foo(x: str) -> int: ...
       |     ^^^
     9 |
    10 | def foo(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default
    ");
}

#[test]
fn test_all_rules_warn_level() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "warn"
invalid-overload-implementation = "warn"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import Any, overload

def any_func(x: Any) -> Any:
    return x + 1

@overload
def overload_func(x: int) -> str: ...

@overload
def overload_func(x: str) -> int: ...

def overload_func(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r#"
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:17
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                 ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:25
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                         ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def overload_func(x: int) -> str: ...
       |     ^^^^^^^^^^^^^
     9 |
    10 | @overload
       |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:11:5
       |
    10 | @overload
    11 | def overload_func(x: str) -> int: ...
       |     ^^^^^^^^^^^^^
    12 |
    13 | def overload_func(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default

    warning[invalid-overload-implementation]: Invalid overload implementation can lead to runtime errors.
      --> test.py:14:5
       |
    13 | def overload_func(x: int | str) -> int | str:
    14 |     return b""
       |     ^^^^^^^^^^
       |
    info: This overload implementation is invalid as `Literal[b""]` is not assignable to any of the overload return types (`str`, `int`)
    info: rule `invalid-overload-implementation` was selected in the configuration file
    "#);
}

#[test]
fn test_mixed_warn_error_levels() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "warn"
invalid-overload-implementation = "error"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import Any, overload

def any_func(x: Any) -> Any:
    return x + 1

@overload
def overload_func(x: int) -> str: ...

@overload
def overload_func(x: str) -> int: ...

def overload_func(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r#"
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:17
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                 ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:25
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                         ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def overload_func(x: int) -> str: ...
       |     ^^^^^^^^^^^^^
     9 |
    10 | @overload
       |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:11:5
       |
    10 | @overload
    11 | def overload_func(x: str) -> int: ...
       |     ^^^^^^^^^^^^^
    12 |
    13 | def overload_func(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default

    error[invalid-overload-implementation]: Invalid overload implementation can lead to runtime errors.
      --> test.py:14:5
       |
    13 | def overload_func(x: int | str) -> int | str:
    14 |     return b""
       |     ^^^^^^^^^^
       |
    info: This overload implementation is invalid as `Literal[b""]` is not assignable to any of the overload return types (`str`, `int`)
    info: rule `invalid-overload-implementation` was selected in the configuration file
    "#);
}

#[test]
fn test_unrecognized_rule_name() {
    let mut runner = TestRunner::new();

    runner.add_file(
        "pyproject.toml",
        r#"
[tool.unsoundness-checker.rules]
typing-any-used = "error"
nonexistent-rule = "error"
invalid-overload-implementation = "warn"
"#,
    );

    runner.add_file(
        "test.py",
        r#"
from typing import Any, overload

def any_func(x: Any) -> Any:
    return x + 1

@overload
def overload_func(x: int) -> str: ...

@overload
def overload_func(x: str) -> int: ...

def overload_func(x: int | str) -> int | str:
    return b""
"#,
    );

    let output = runner.run_test();
    insta::assert_snapshot!(output, @r#"
    warning[unknown-rule]: Unknown lint rule `nonexistent-rule`
     --> pyproject.toml:4:1
      |
    2 | [tool.unsoundness-checker.rules]
    3 | typing-any-used = "error"
    4 | nonexistent-rule = "error"
      | ^^^^^^^^^^^^^^^^
    5 | invalid-overload-implementation = "warn"
      |

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:17
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                 ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:25
      |
    2 | from typing import Any, overload
    3 |
    4 | def any_func(x: Any) -> Any:
      |                         ^^^
    5 |     return x + 1
      |
    info: rule `typing-any-used` was selected in the configuration file

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:8:5
       |
     7 | @overload
     8 | def overload_func(x: int) -> str: ...
       |     ^^^^^^^^^^^^^
     9 |
    10 | @overload
       |
    info: rule `typing-overload-used` is enabled by default

    warning[typing-overload-used]: Using `typing.overload` can lead to runtime errors.
      --> test.py:11:5
       |
    10 | @overload
    11 | def overload_func(x: str) -> int: ...
       |     ^^^^^^^^^^^^^
    12 |
    13 | def overload_func(x: int | str) -> int | str:
       |
    info: rule `typing-overload-used` is enabled by default

    warning[invalid-overload-implementation]: Invalid overload implementation can lead to runtime errors.
      --> test.py:14:5
       |
    13 | def overload_func(x: int | str) -> int | str:
    14 |     return b""
       |     ^^^^^^^^^^
       |
    info: This overload implementation is invalid as `Literal[b""]` is not assignable to any of the overload return types (`str`, `int`)
    info: rule `invalid-overload-implementation` was selected in the configuration file
    "#);
}
