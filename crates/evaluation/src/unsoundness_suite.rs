use anyhow::{Context, Result};

use colored::Colorize;
use ruff_db::diagnostic::DiagnosticId;
use ruff_db::system::{OsSystem, SystemPath, SystemPathBuf};
use ruff_python_ast::PythonVersion;
use ty_project::metadata::Options;
use ty_project::metadata::options::EnvironmentOptions;
use ty_project::metadata::value::RangedValue;
use ty_project::{Db, ProjectDatabase, ProjectMetadata};
use unsoundness_checker::checker::check_project;
use unsoundness_checker::rules::{self};

use crate::real_world_projects::{InstalledProject, RealWorldProject};

struct UnsoundnessSuiteFile {
    path: &'static str,
    expected_diagnostics: DiagnosticId,
}

/// Files that show unsoundness examples that we should catch, and what rule they violate.
const UNSOUNDNESS_EXAMPLE_FILES: &[UnsoundnessSuiteFile] = &[
    UnsoundnessSuiteFile {
        path: "examples/Any/any.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPING_ANY_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/Any/callable_ellipsis.py",
        expected_diagnostics: DiagnosticId::Lint(rules::CALLABLE_ELLIPSIS_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/TypeIs/incorrect_typeis.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPING_TYPE_IS_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/TypeIs/typeis_float.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPING_TYPE_IS_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/TypeIs/typeis_list_any.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPING_TYPE_IS_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/TypeIs/typeis_narrow_list.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPING_TYPE_IS_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/directives/cast.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPING_CAST_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/directives/if_typechecking.py",
        expected_diagnostics: DiagnosticId::Lint(rules::IF_TYPE_CHECKING_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/directives/type_ignore.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPE_CHECKING_DIRECTIVE_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/generic/mutable_default.py",
        expected_diagnostics: DiagnosticId::Lint(rules::MUTABLE_GENERIC_DEFAULT.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/overload/incorrect_impl.py",
        expected_diagnostics: DiagnosticId::Lint(rules::INVALID_OVERLOAD_IMPLEMENTATION.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/overload/incorrect_impl.py",
        expected_diagnostics: DiagnosticId::Lint(rules::TYPING_OVERLOAD_USED.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/runtime/defaults.py",
        expected_diagnostics: DiagnosticId::Lint(rules::INVALID_FUNCTION_DEFAULTS.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/runtime/dunder_variables.py",
        expected_diagnostics: DiagnosticId::Lint(rules::MANGLED_DUNDER_INSTANCE_VARIABLE.name()),
    },
    UnsoundnessSuiteFile {
        path: "examples/runtime/func_code.py",
        expected_diagnostics: DiagnosticId::Lint(rules::MUTATING_FUNCTION_CODE_ATTRIBUTE.name()),
    },
];

const UNSOUNDNESS_SUITE: RealWorldProject = RealWorldProject {
    name: "jelle_unsoundness",
    repository: "https://github.com/JelleZijlstra/unsoundness",
    commit: "c711b773567765a4b0331993467d19ed0f494161",
    paths: &[],
    dependencies: &[],
    python_version: PythonVersion::PY313,
};

pub fn run_unsoundness_suite() -> Result<()> {
    let installed_project = UNSOUNDNESS_SUITE.setup()?;

    let mut passed = true;

    for file in UNSOUNDNESS_EXAMPLE_FILES {
        passed &= run_file(file, &installed_project)?;
    }

    println!();

    if passed {
        println!("{}", "All unsoundness tests passed!".green());
    } else {
        println!("{}", "Some unsoundness tests failed!".red());
    }

    Ok(())
}

fn run_file(file: &UnsoundnessSuiteFile, installed_project: &InstalledProject) -> Result<bool> {
    let root = SystemPathBuf::from_path_buf(installed_project.path.clone())
        .map_err(|p| anyhow::anyhow!("Failed to convert path to SystemPathBuf: {}", p.display()))?;
    let system = OsSystem::new(&root);

    let mut metadata =
        ProjectMetadata::discover(&root, &system).context("Failed to discover project metadata")?;

    metadata.apply_options(Options {
        environment: Some(EnvironmentOptions {
            python_version: Some(RangedValue::cli(installed_project.config.python_version)),
            ..EnvironmentOptions::default()
        }),
        ..Options::default()
    });

    let rules = metadata.options().rules.clone();
    let mut db =
        ProjectDatabase::new(metadata, system).context("Failed to create project database")?;

    let check_path = vec![SystemPath::absolute(file.path, &root)];

    db.project().set_included_paths(&mut db, check_path);

    let rule_registry = unsoundness_checker::default_rule_registry();
    let (rule_selection, _rule_diagnostics) =
        unsoundness_checker::rule::RuleSelection::from_rules_selection(
            rule_registry,
            rules.as_ref(),
            &db,
        );

    let diagnostics = check_project(&db, &rule_selection);

    for diagnostic in &diagnostics {
        if diagnostic.id() == file.expected_diagnostics {
            println!(
                "Found expected rule `{}` for file `{}`",
                diagnostic.id(),
                file.path
            );
            return Ok(true);
        }
    }

    println!(
        "Expected rule `{}` not found for file `{}`",
        file.expected_diagnostics, file.path
    );

    Ok(false)
}
