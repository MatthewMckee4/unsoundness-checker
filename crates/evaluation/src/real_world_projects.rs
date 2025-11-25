#![allow(clippy::print_stderr)]

use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use anyhow::{Context, Result};
use ruff_db::system::{SystemPath, SystemPathBuf};
use ruff_python_ast::PythonVersion;
use unsoundness_checker::checker::check_project;

/// Configuration for a real-world project to benchmark
#[derive(Debug, Clone)]
pub struct RealWorldProject<'a> {
    /// The name of the project.
    pub name: &'a str,
    /// The project's GIT repository. Must be publicly accessible.
    pub repository: &'a str,
    /// Specific commit hash to checkout
    pub commit: &'a str,
    /// List of paths within the project to check
    pub paths: &'a [&'a str],
    /// Dependencies to install via uv
    pub dependencies: &'a [&'a str],
    /// Python version to use
    pub python_version: PythonVersion,
}

impl<'a> RealWorldProject<'a> {
    /// Setup a real-world project for benchmarking
    pub fn setup(self) -> Result<InstalledProject<'a>> {
        let start = Instant::now();
        tracing::debug!("Setting up project {}", self.name);

        // Create project directory in cargo target
        let project_root = get_project_cache_dir(self.name)?;

        // Clone the repository if it doesn't exist, or update if it does
        if project_root.exists() {
            tracing::debug!("Updating repository for project '{}'...", self.name);
            let start = std::time::Instant::now();
            update_repository(&project_root, self.commit)?;
            tracing::debug!(
                "Repository update completed in {:.2}s",
                start.elapsed().as_secs_f64()
            );
        } else {
            tracing::debug!("Cloning repository for project '{}'...", self.name);
            let start = std::time::Instant::now();
            clone_repository(self.repository, &project_root, self.commit)?;
            tracing::debug!(
                "Repository clone completed in {:.2}s",
                start.elapsed().as_secs_f64()
            );
        }

        let checkout = Checkout {
            path: project_root,
            project: self,
        };

        // Install dependencies if specified
        tracing::debug!(
            "Installing {} dependencies for project '{}'...",
            checkout.project().dependencies.len(),
            checkout.project().name
        );
        let start_install = std::time::Instant::now();
        install_dependencies(&checkout)?;
        tracing::debug!(
            "Dependency installation completed in {:.2}s",
            start_install.elapsed().as_secs_f64()
        );

        tracing::debug!("Project setup took: {:.2}s", start.elapsed().as_secs_f64());

        Ok(InstalledProject {
            path: checkout.path,
            config: checkout.project,
        })
    }
}

struct Checkout<'a> {
    project: RealWorldProject<'a>,
    path: PathBuf,
}

impl<'a> Checkout<'a> {
    /// Get the virtual environment path
    fn venv_path(&self) -> PathBuf {
        self.path.join(".venv")
    }

    const fn project(&self) -> &RealWorldProject<'a> {
        &self.project
    }
}

/// Checked out project with its dependencies installed.
pub struct InstalledProject<'a> {
    /// Path to the cloned project
    pub path: PathBuf,
    /// Project configuration
    pub config: RealWorldProject<'a>,
}

impl InstalledProject<'_> {
    /// Get the benchmark paths
    pub fn check_paths(&self) -> Vec<PathBuf> {
        self.config
            .paths
            .iter()
            .map(|p| self.path.join(p))
            .collect()
    }
}

/// Get the cache directory for a project in the cargo target directory
fn get_project_cache_dir(project_name: &str) -> Result<std::path::PathBuf> {
    let target_dir = cargo_target_directory()
        .cloned()
        .unwrap_or_else(|| PathBuf::from("target"));
    let target_dir =
        std::path::absolute(target_dir).context("Failed to construct an absolute path")?;
    let cache_dir = target_dir.join("benchmark_cache").join(project_name);

    if let Some(parent) = cache_dir.parent() {
        std::fs::create_dir_all(parent).context("Failed to create cache directory")?;
    }

    Ok(cache_dir)
}

/// Update an existing repository
fn update_repository(project_root: &Path, commit: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["fetch", "origin", commit])
        .current_dir(project_root)
        .output()
        .context("Failed to execute git fetch command")?;

    if !output.status.success() {
        anyhow::bail!(
            "Git fetch of commit {} failed: {}",
            commit,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Checkout specific commit
    let output = Command::new("git")
        .args(["checkout", commit])
        .current_dir(project_root)
        .output()
        .context("Failed to execute git checkout command")?;

    anyhow::ensure!(
        output.status.success(),
        "Git checkout of commit {} failed: {}",
        commit,
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}

/// Clone a git repository to the specified directory
fn clone_repository(repo_url: &str, target_dir: &Path, commit: &str) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = target_dir.parent() {
        std::fs::create_dir_all(parent).context("Failed to create parent directory for clone")?;
    }

    // Clone with minimal depth and fetch only the specific commit
    let output = Command::new("git")
        .args([
            "clone",
            "--filter=blob:none", // Don't download large files initially
            "--no-checkout",      // Don't checkout files yet
            repo_url,
            target_dir.to_str().unwrap(),
        ])
        .output()
        .context("Failed to execute git clone command")?;

    anyhow::ensure!(
        output.status.success(),
        "Git clone failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Fetch the specific commit
    let output = Command::new("git")
        .args(["fetch", "origin", commit])
        .current_dir(target_dir)
        .output()
        .context("Failed to execute git fetch command")?;

    anyhow::ensure!(
        output.status.success(),
        "Git fetch of commit {} failed: {}",
        commit,
        String::from_utf8_lossy(&output.stderr)
    );

    // Checkout the specific commit
    let output = Command::new("git")
        .args(["checkout", commit])
        .current_dir(target_dir)
        .output()
        .context("Failed to execute git checkout command")?;

    anyhow::ensure!(
        output.status.success(),
        "Git checkout of commit {} failed: {}",
        commit,
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}

/// Install dependencies using uv with date constraints
fn install_dependencies(checkout: &Checkout) -> Result<()> {
    // Check if uv is available
    let uv_check = Command::new("uv")
        .arg("--version")
        .output()
        .context("Failed to execute uv version check.")?;

    if !uv_check.status.success() {
        anyhow::bail!(
            "uv is not installed or not found in PATH. If you need to install it, follow the instructions at https://docs.astral.sh/uv/getting-started/installation/"
        );
    }

    let venv_path = checkout.venv_path();
    let python_version_str = checkout.project().python_version.to_string();

    let output = Command::new("uv")
        .args(["venv", "--python", &python_version_str, "--allow-existing"])
        .arg(&venv_path)
        .output()
        .context("Failed to execute uv venv command")?;

    anyhow::ensure!(
        output.status.success(),
        "Failed to create virtual environment: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    if checkout.project().dependencies.is_empty() {
        tracing::debug!(
            "No dependencies to install for project '{}'",
            checkout.project().name
        );
        return Ok(());
    }

    // Install dependencies with date constraint in the isolated environment
    let mut cmd = Command::new("uv");
    cmd.args(["pip", "install", "--python", venv_path.to_str().unwrap()])
        .args(checkout.project().dependencies);

    let output = cmd
        .output()
        .context("Failed to execute uv pip install command")?;

    anyhow::ensure!(
        output.status.success(),
        "Dependency installation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}

static CARGO_TARGET_DIR: std::sync::OnceLock<Option<PathBuf>> = std::sync::OnceLock::new();

fn cargo_target_directory() -> Option<&'static PathBuf> {
    CARGO_TARGET_DIR
        .get_or_init(|| {
            #[derive(serde::Deserialize)]
            struct Metadata {
                target_directory: PathBuf,
            }

            std::env::var_os("CARGO_TARGET_DIR")
                .map(PathBuf::from)
                .or_else(|| {
                    let output = Command::new(std::env::var_os("CARGO")?)
                        .args(["metadata", "--format-version", "1"])
                        .output()
                        .ok()?;
                    let metadata: Metadata = serde_json::from_slice(&output.stdout).ok()?;
                    Some(metadata.target_directory)
                })
        })
        .as_ref()
}

/// Benchmark configuration for a real-world project
pub struct Benchmark<'a> {
    project: RealWorldProject<'a>,
}

impl<'a> Benchmark<'a> {
    pub const fn pydantic() -> Self {
        Self {
            project: RealWorldProject {
                name: "pydantic",
                repository: "https://github.com/pydantic/pydantic",
                commit: "0c4a22b64b23dfad27387750cf07487efc45eb05",
                paths: &["pydantic"],
                dependencies: &[
                    "annotated-types",
                    "pydantic-core",
                    "typing-extensions",
                    "typing-inspection",
                ],
                python_version: PythonVersion::PY313,
            },
        }
    }

    pub const fn pytest() -> Self {
        Self {
            project: RealWorldProject {
                name: "pytest",
                repository: "https://github.com/pytest-dev/pytest",
                commit: "94f4922d9a73236d88d637e71316ceb446695158",
                paths: &["src/_pytest"],
                dependencies: &["iniconfig", "packaging", "pluggy", "exceptiongroup"],
                python_version: PythonVersion::PY313,
            },
        }
    }

    pub const fn fastapi() -> Self {
        Self {
            project: RealWorldProject {
                name: "fastapi",
                repository: "https://github.com/fastapi/fastapi",
                commit: "5b0625df96e4ea11b54fcb2a76f21f7ad94764fe",
                paths: &["fastapi"],
                dependencies: &["starlette", "pydantic", "typing-extensions"],
                python_version: PythonVersion::PY313,
            },
        }
    }

    pub const fn black() -> Self {
        Self {
            project: RealWorldProject {
                name: "black",
                repository: "https://github.com/psf/black",
                commit: "cde9494ac5b89bd4c9154f746a543685961983a8",
                paths: &["src/black"],
                dependencies: &[
                    "click",
                    "mypy-extensions",
                    "pathspec",
                    "platformdirs",
                    "packaging",
                ],
                python_version: PythonVersion::PY313,
            },
        }
    }

    pub const fn flask() -> Self {
        Self {
            project: RealWorldProject {
                name: "flask",
                repository: "https://github.com/pallets/flask",
                commit: "2579ce9f18e67ec3213c6eceb5240310ccd46af8",
                paths: &["src/flask"],
                dependencies: &["werkzeug", "jinja2", "itsdangerous", "click"],
                python_version: PythonVersion::PY39,
            },
        }
    }

    /// Get the project name
    pub const fn project_name(&self) -> &str {
        self.project.name
    }

    /// Setup the project (clone and install dependencies)
    pub fn setup(&self) -> Result<InstalledProject<'a>> {
        self.project.clone().setup()
    }
}

/// Run the unsoundness checker on the installed project
pub fn run_checker(installed_project: &InstalledProject) -> Result<usize> {
    use ruff_db::system::OsSystem;
    use ty_project::{
        Db, ProjectDatabase, ProjectMetadata,
        metadata::{
            Options,
            options::EnvironmentOptions,
            value::{RangedValue, RelativePathBuf},
        },
    };

    let root = SystemPathBuf::from_path_buf(installed_project.path.clone())
        .map_err(|p| anyhow::anyhow!("Failed to convert path to SystemPathBuf: {}", p.display()))?;
    let system = OsSystem::new(&root);

    let mut metadata =
        ProjectMetadata::discover(&root, &system).context("Failed to discover project metadata")?;

    metadata.apply_options(Options {
        environment: Some(EnvironmentOptions {
            python_version: Some(RangedValue::cli(installed_project.config.python_version)),
            python: Some(RelativePathBuf::cli(SystemPath::new(".venv"))),
            ..EnvironmentOptions::default()
        }),
        ..Options::default()
    });

    // Get rules before moving metadata
    let rules = metadata.options().rules.clone();

    let mut db =
        ProjectDatabase::new(metadata, system).context("Failed to create project database")?;

    let check_paths: Vec<_> = installed_project
        .check_paths()
        .iter()
        .filter_map(|path| SystemPathBuf::from_path_buf(path.clone()).ok())
        .map(|path| SystemPath::absolute(&path, &root))
        .collect();

    db.project().set_included_paths(&mut db, check_paths);

    // Get all files in the project

    // Get the default rule registry and selection
    let rule_registry = unsoundness_checker::default_rule_registry();
    let (rule_selection, _rule_diagnostics) =
        unsoundness_checker::rule::RuleSelection::from_rules_selection(
            rule_registry,
            rules.as_ref(),
            &db,
        );

    let diagnostics = check_project(&db, &rule_selection);

    Ok(diagnostics.len())
}
