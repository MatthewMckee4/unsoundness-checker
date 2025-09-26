use std::fs;

use ruff_db::{
    diagnostic::{DisplayDiagnosticConfig, DisplayDiagnostics},
    system::{OsSystem, SystemPathBuf},
};
use tempfile::TempDir;
use ty_project::{
    Db, ProjectDatabase, ProjectMetadata, metadata::options::ProjectOptionsOverrides,
};
use unsoundness_checker::checker::check_file;

/// Helper function to create a test project and check Python code, returning formatted diagnostics
fn check_python_code(code: &str) -> String {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();

    let python_file = temp_path.join("test.py");
    fs::write(&python_file, code).expect("Failed to write test file");

    let cwd =
        SystemPathBuf::from_path_buf(temp_path.to_path_buf()).expect("Failed to convert path");
    let system = OsSystem::new(&cwd);

    let mut project_metadata =
        ProjectMetadata::discover(&cwd, &system).expect("Failed to discover project");
    project_metadata
        .apply_configuration_files(&system)
        .expect("Failed to apply config");

    let project_options_overrides = ProjectOptionsOverrides::default();
    project_metadata.apply_overrides(&project_options_overrides);

    let db = ProjectDatabase::new(project_metadata, system).expect("Failed to create database");

    let files = db.project().files(&db);
    let mut diagnostics = Vec::new();

    for file in &files {
        diagnostics.extend(check_file(&db, file));
    }

    let display_config = DisplayDiagnosticConfig::default();
    let display = DisplayDiagnostics::new(&db, &display_config, &diagnostics);

    format!("{display}")
}

#[test]
fn test_typing_any_detected() {
    let code = r"
from typing import Any

def foo(x: Any) -> str:
    return str(x)
";

    let output = check_python_code(code);
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

    let output = check_python_code(code);
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

    let output = check_python_code(code);
    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_any_in_return_type_not_detected() {
    let code = r"
from typing import Any

def foo(x: str) -> Any:
    return x
";

    let output = check_python_code(code);
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

    let output = check_python_code(code);
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

    let output = check_python_code(code);
    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_mixed_annotations() {
    let code = r"
from typing import Any, List, Optional

def complex_function(
    required_param: str,
    any_param: Any,
    optional_param: Optional[int] = None,
    list_param: List[str] = None
) -> bool:
    return True
";

    let output = check_python_code(code);
    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:6:16
      |
    4 | def complex_function(
    5 |     required_param: str,
    6 |     any_param: Any,
      |                ^^^
    7 |     optional_param: Optional[int] = None,
    8 |     list_param: List[str] = None
      |
    info: rule `typing-any-used` is enabled by default
    ");
}

#[test]
fn test_lambda_with_any() {
    let code = r"
from typing import Any

lambda_func = lambda x: Any, y: str: str(x) + y
";

    let output = check_python_code(code);
    insta::assert_snapshot!(output, @"");
}

#[test]
fn test_async_function_with_any() {
    let code = r"
from typing import Any
import asyncio

async def async_foo(param: Any) -> None:
    await asyncio.sleep(0.1)
    print(param)
";

    let output = check_python_code(code);
    insta::assert_snapshot!(output, @r"
    error[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:5:28
      |
    3 | import asyncio
    4 |
    5 | async def async_foo(param: Any) -> None:
      |                            ^^^
    6 |     await asyncio.sleep(0.1)
    7 |     print(param)
      |
    info: rule `typing-any-used` is enabled by default
    ");
}
