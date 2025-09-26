use crate::TestRunner;

#[test]
fn test_typing_any_detected() {
    let code = r"
from typing import Any

def foo(x: Any) -> str:
    return str(x)
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:12
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any) -> str:
      |            ^^^
    5 |     return str(x)
      |
    info: rule `typing-any-used` is enabled by default
    ");
}

#[test]
fn test_multiple_any_parameters() {
    let code = r"
from typing import Any

def foo(x: Any, y: Any, z: str) -> None:
    pass
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:12
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any, y: Any, z: str) -> None:
      |            ^^^
    5 |     pass
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:20
      |
    2 | from typing import Any
    3 |
    4 | def foo(x: Any, y: Any, z: str) -> None:
      |                    ^^^
    5 |     pass
      |
    info: rule `typing-any-used` is enabled by default
    ");
}

#[test]
fn test_no_any_usage() {
    let code = r"
def foo(x: str, y: int) -> bool:
    return len(x) == y
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_any_in_return_type_not_detected() {
    let code = r"
from typing import Any

def foo(x: str) -> Any:
    return x
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_nested_function_with_any() {
    let code = r"
from typing import Any

def outer():
    def inner(param: Any) -> str:
        return str(param)
    return inner
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_class_method_with_any() {
    let code = r"
from typing import Any

class TestClass:
    def method(self, param: Any) -> None:
        pass

    @staticmethod
    def static_method(param: Any) -> str:
        return str(param)
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_mixed_annotations() {
    let code = r"
from typing import Any

def complex_function(
    required_param: str,
    any_param: Any,
    optional_param: Any | None = None,
    list_param: list[Any] | None = None
) -> bool:
    return True
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:6:16
      |
    4 | def complex_function(
    5 |     required_param: str,
    6 |     any_param: Any,
      |                ^^^
    7 |     optional_param: Any | None = None,
    8 |     list_param: list[Any] | None = None
      |
    info: rule `typing-any-used` is enabled by default
    ");
}
