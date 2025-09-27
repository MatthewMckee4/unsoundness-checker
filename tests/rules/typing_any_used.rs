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
fn test_nested_function_with_any() {
    let code = r"
from typing import Any

def outer():
    def inner(param: Any) -> str:
        return str(param)
    return inner
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:5:22
      |
    4 | def outer():
    5 |     def inner(param: Any) -> str:
      |                      ^^^
    6 |         return str(param)
    7 |     return inner
      |
    info: rule `typing-any-used` is enabled by default
    ");
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

    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:5:29
      |
    4 | class TestClass:
    5 |     def method(self, param: Any) -> None:
      |                             ^^^
    6 |         pass
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
      --> test.py:9:30
       |
     8 |     @staticmethod
     9 |     def static_method(param: Any) -> str:
       |                              ^^^
    10 |         return str(param)
       |
    info: rule `typing-any-used` is enabled by default
    ");
}

#[test]
fn test_mixed_annotations() {
    let code = r"
from typing import Any

def complex_function(
    required_param: str,
    any_param: Any,
    optional_param: Any | None,
    list_param: list[Any] | None,
    dict_param: dict[str, Any] | None,
    nested: list[dict[str, list[set[tuple[str, Any]]]] | None]
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
    7 |     optional_param: Any | None,
    8 |     list_param: list[Any] | None,
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:7:21
      |
    5 |     required_param: str,
    6 |     any_param: Any,
    7 |     optional_param: Any | None,
      |                     ^^^
    8 |     list_param: list[Any] | None,
    9 |     dict_param: dict[str, Any] | None,
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
      --> test.py:8:22
       |
     6 |     any_param: Any,
     7 |     optional_param: Any | None,
     8 |     list_param: list[Any] | None,
       |                      ^^^
     9 |     dict_param: dict[str, Any] | None,
    10 |     nested: list[dict[str, list[set[tuple[str, Any]]]] | None]
       |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
      --> test.py:9:27
       |
     7 |     optional_param: Any | None,
     8 |     list_param: list[Any] | None,
     9 |     dict_param: dict[str, Any] | None,
       |                           ^^^
    10 |     nested: list[dict[str, list[set[tuple[str, Any]]]] | None]
    11 | ) -> bool:
       |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
      --> test.py:10:48
       |
     8 |     list_param: list[Any] | None,
     9 |     dict_param: dict[str, Any] | None,
    10 |     nested: list[dict[str, list[set[tuple[str, Any]]]] | None]
       |                                                ^^^
    11 | ) -> bool:
    12 |     return True
       |
    info: rule `typing-any-used` is enabled by default
    ");
}

#[test]
fn test_any_in_return_type() {
    let code = r"
from typing import Any

def foo() -> Any:
    return 42
";

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:14
      |
    2 | from typing import Any
    3 |
    4 | def foo() -> Any:
      |              ^^^
    5 |     return 42
      |
    info: rule `typing-any-used` is enabled by default
    ");
}

#[test]
fn test_any_in_nested_return_type() {
    let code = r#"
from typing import Any

def get_list() -> list[Any]:
    return [1, "hello", 3.14]

def get_dict() -> dict[str, Any]:
    return {"key": "value"}

def get_complex() -> dict[str, list[Any]]:
    return {"items": [1, 2, 3]}
"#;

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r#"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:24
      |
    2 | from typing import Any
    3 |
    4 | def get_list() -> list[Any]:
      |                        ^^^
    5 |     return [1, "hello", 3.14]
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:7:29
      |
    5 |     return [1, "hello", 3.14]
    6 |
    7 | def get_dict() -> dict[str, Any]:
      |                             ^^^
    8 |     return {"key": "value"}
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
      --> test.py:10:37
       |
     8 |     return {"key": "value"}
     9 |
    10 | def get_complex() -> dict[str, list[Any]]:
       |                                     ^^^
    11 |     return {"items": [1, 2, 3]}
       |
    info: rule `typing-any-used` is enabled by default
    "#);
}

#[test]
fn test_variable_annotation_expressions() {
    let code = r#"
from typing import Any

a: Any = 1
b: Any = "hello"
c: Any = None
"#;

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r#"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:4
      |
    2 | from typing import Any
    3 |
    4 | a: Any = 1
      |    ^^^
    5 | b: Any = "hello"
    6 | c: Any = None
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:5:4
      |
    4 | a: Any = 1
    5 | b: Any = "hello"
      |    ^^^
    6 | c: Any = None
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:6:4
      |
    4 | a: Any = 1
    5 | b: Any = "hello"
    6 | c: Any = None
      |    ^^^
      |
    info: rule `typing-any-used` is enabled by default
    "#);
}

#[test]
fn test_nested_annotation_expressions() {
    let code = r#"
from typing import Any

items: list[Any] = [1, 2, 3]
mapping: dict[str, Any] = {"key": "value"}
nested: dict[str, list[Any]] = {"items": [1, 2]}
"#;

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r#"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:13
      |
    2 | from typing import Any
    3 |
    4 | items: list[Any] = [1, 2, 3]
      |             ^^^
    5 | mapping: dict[str, Any] = {"key": "value"}
    6 | nested: dict[str, list[Any]] = {"items": [1, 2]}
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:5:20
      |
    4 | items: list[Any] = [1, 2, 3]
    5 | mapping: dict[str, Any] = {"key": "value"}
      |                    ^^^
    6 | nested: dict[str, list[Any]] = {"items": [1, 2]}
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:6:24
      |
    4 | items: list[Any] = [1, 2, 3]
    5 | mapping: dict[str, Any] = {"key": "value"}
    6 | nested: dict[str, list[Any]] = {"items": [1, 2]}
      |                        ^^^
      |
    info: rule `typing-any-used` is enabled by default
    "#);
}

#[test]
fn test_class_attribute_annotation_expressions() {
    let code = r#"
from typing import Any

class MyClass:
    attr: Any = "default"
    values: list[Any] = []

    def __init__(self):
        self.data: Any = None
        self.items: dict[str, Any] = {}
"#;

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r#"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:5:11
      |
    4 | class MyClass:
    5 |     attr: Any = "default"
      |           ^^^
    6 |     values: list[Any] = []
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:6:18
      |
    4 | class MyClass:
    5 |     attr: Any = "default"
    6 |     values: list[Any] = []
      |                  ^^^
    7 |
    8 |     def __init__(self):
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
      --> test.py:9:20
       |
     8 |     def __init__(self):
     9 |         self.data: Any = None
       |                    ^^^
    10 |         self.items: dict[str, Any] = {}
       |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
      --> test.py:10:31
       |
     8 |     def __init__(self):
     9 |         self.data: Any = None
    10 |         self.items: dict[str, Any] = {}
       |                               ^^^
       |
    info: rule `typing-any-used` is enabled by default
    "#);
}

#[test]
fn test_deeply_nested_any_in_return_type() {
    let code = r#"
from typing import Any

def deeply_nested() -> dict[str, list[tuple[str, Any]]]:
    return {"data": [("key", 42)]}

def ultra_nested() -> list[dict[str, set[tuple[Any, str]]]]:
    return [{"items": {("value", "key")}}]
"#;

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r#"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:50
      |
    2 | from typing import Any
    3 |
    4 | def deeply_nested() -> dict[str, list[tuple[str, Any]]]:
      |                                                  ^^^
    5 |     return {"data": [("key", 42)]}
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:7:48
      |
    5 |     return {"data": [("key", 42)]}
    6 |
    7 | def ultra_nested() -> list[dict[str, set[tuple[Any, str]]]]:
      |                                                ^^^
    8 |     return [{"items": {("value", "key")}}]
      |
    info: rule `typing-any-used` is enabled by default
    "#);
}

#[test]
fn test_any_in_union_return_type() {
    let code = r#"
from typing import Any

def union_with_any() -> Any | str:
    return "hello"

def complex_union() -> dict[str, Any] | list[Any] | None:
    return None
"#;

    let output = TestRunner::from_file("test.py", code).run_test();

    insta::assert_snapshot!(output, @r#"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:25
      |
    2 | from typing import Any
    3 |
    4 | def union_with_any() -> Any | str:
      |                         ^^^
    5 |     return "hello"
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:7:34
      |
    5 |     return "hello"
    6 |
    7 | def complex_union() -> dict[str, Any] | list[Any] | None:
      |                                  ^^^
    8 |     return None
      |
    info: rule `typing-any-used` is enabled by default

    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:7:46
      |
    5 |     return "hello"
    6 |
    7 | def complex_union() -> dict[str, Any] | list[Any] | None:
      |                                              ^^^
    8 |     return None
      |
    info: rule `typing-any-used` is enabled by default
    "#);
}
