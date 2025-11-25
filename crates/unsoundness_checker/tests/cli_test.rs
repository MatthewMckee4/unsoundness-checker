#![allow(clippy::significant_drop_tightening)]

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use insta::{Settings, internals::SettingsBindDropGuard};
use insta_cmd::assert_cmd_snapshot;
use tempfile::TempDir;

/// CLI test context for running the unsoundness checker binary
#[allow(dead_code)]
pub(crate) struct CliTest {
    _temp_dir: TempDir,
    settings: Settings,
    settings_scope: Option<SettingsBindDropGuard>,
    project_dir: PathBuf,
    binary_path: PathBuf,
}

#[allow(dead_code)]
impl CliTest {
    pub(crate) fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();

        // Get the project directory - simplified to work cross-platform
        let project_dir = temp_dir.path().canonicalize().unwrap();

        let mut settings = Settings::clone_current();

        // Add filters for consistent snapshots across platforms
        settings.add_filter(&tempdir_filter(&project_dir), "[TEMP_DIR]/");
        // Normalize Windows backslashes to forward slashes
        settings.add_filter(r"\\", "/");
        // Filter out timing information
        settings.add_filter(r"\d+\.\d+s", "0.000s");

        let settings_scope = settings.bind_to_scope();

        let binary_path = get_binary_path();

        Self {
            project_dir,
            _temp_dir: temp_dir,
            settings,
            settings_scope: Some(settings_scope),
            binary_path,
        }
    }

    pub(crate) fn with_file(path: impl AsRef<Path>, content: &str) -> Self {
        let test = Self::new();
        test.write_file(path, content);
        test
    }

    pub(crate) fn write_file(&self, path: impl AsRef<Path>, content: &str) {
        let path = path.as_ref();
        let file_path = self.project_dir.join(path);

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        std::fs::write(&file_path, content).unwrap();
    }

    pub(crate) fn write_files<'a>(&self, files: impl IntoIterator<Item = (&'a str, &'a str)>) {
        for (path, content) in files {
            self.write_file(path, content);
        }
    }

    /// Add a filter to the settings and rebind them.
    pub(crate) fn with_filter(mut self, pattern: &str, replacement: &str) -> Self {
        self.settings.add_filter(pattern, replacement);
        drop(self.settings_scope.take());
        self.settings_scope = Some(self.settings.bind_to_scope());
        self
    }

    pub(crate) fn root(&self) -> &Path {
        &self.project_dir
    }

    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new(&self.binary_path);
        command.current_dir(&self.project_dir).arg("check");
        command.env_clear();
        command
    }
}

fn tempdir_filter(path: &Path) -> String {
    format!(r"{}/?", regex::escape(&path.display().to_string()))
}

fn get_binary_path() -> PathBuf {
    // Use CARGO_BIN_EXE environment variable set by cargo test
    PathBuf::from(env!("CARGO_BIN_EXE_unsoundness_checker"))
}

#[test]
fn test_summary_false_default() {
    let test = CliTest::with_file(
        "test.py",
        r"
from typing import Any

x: Any = 1
",
    );

    assert_cmd_snapshot!(test.command(), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:4
      |
    2 | from typing import Any
    3 |
    4 | x: Any = 1
      |    ^^^
      |
    info: rule `typing-any-used` is enabled by default

    Found 1 diagnostic

    ----- stderr -----
    ");
}

#[test]
fn test_summary_one_rule() {
    let test = CliTest::with_file(
        "test.py",
        r"
from typing import Any

x: Any = 1
",
    );

    assert_cmd_snapshot!(test.command().arg("--summary").arg("false"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:4
      |
    2 | from typing import Any
    3 |
    4 | x: Any = 1
      |    ^^^
      |
    info: rule `typing-any-used` is enabled by default

    Found 1 diagnostic

    ----- stderr -----
    ");

    assert_cmd_snapshot!(test.command().arg("--summary").arg("true"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:4
      |
    2 | from typing import Any
    3 |
    4 | x: Any = 1
      |    ^^^
      |
    info: rule `typing-any-used` is enabled by default

    summary:
      typing-any-used: 1

    Found 1 diagnostic

    ----- stderr -----
    ");

    assert_cmd_snapshot!(test.command().arg("--summary").arg("only"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    Summary:
      typing-any-used: 1

    Total: 1 diagnostic

    ----- stderr -----
    ");
}

#[test]
fn test_summary_multiple_rules() {
    let test = CliTest::new();

    test.write_file(
        "test.py",
        r"
from typing import Any, Callable

x: Any = 1
y: Callable[..., str] = lambda s: s.upper()
",
    );

    assert_cmd_snapshot!(test.command().arg("--summary").arg("true"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    warning[typing-any-used]: Using `typing.Any` in type annotations can lead to runtime errors.
     --> test.py:4:4
      |
    2 | from typing import Any, Callable
    3 |
    4 | x: Any = 1
      |    ^^^
    5 | y: Callable[..., str] = lambda s: s.upper()
      |
    info: rule `typing-any-used` is enabled by default

    warning[callable-ellipsis-used]: Using `...` in `Callable` type annotations can lead to runtime type errors.
     --> test.py:5:4
      |
    4 | x: Any = 1
    5 | y: Callable[..., str] = lambda s: s.upper()
      |    ^^^^^^^^^^^^^^^^^^
      |
    info: rule `callable-ellipsis-used` is enabled by default

    summary:
      callable-ellipsis-used: 1
      typing-any-used: 1

    Found 2 diagnostics

    ----- stderr -----
    ");

    assert_cmd_snapshot!(test.command().arg("--summary").arg("only"), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    Summary:
      callable-ellipsis-used: 1
      typing-any-used: 1

    Total: 2 diagnostics

    ----- stderr -----
    ");
}

#[test]
fn test_summary_no_diagnostics() {
    let test = CliTest::with_file(
        "test.py",
        r"
def add(x: int, y: int) -> int:
    return x + y
",
    );

    assert_cmd_snapshot!(test.command().arg("--summary").arg("true"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    All checks passed

    ----- stderr -----
    ");
}
