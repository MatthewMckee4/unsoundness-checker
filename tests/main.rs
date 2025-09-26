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

pub struct TestRunner {
    temp_dir: TempDir,
}

impl TestRunner {
    #[must_use]
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        Self { temp_dir }
    }

    #[must_use]
    pub fn from_file(filename: &str, content: &str) -> Self {
        let mut runner = Self::new();
        runner.add_file(filename, content);
        runner
    }

    #[must_use]
    pub fn from_files(files: Vec<(&str, &str)>) -> Self {
        let mut runner = Self::new();
        for (filename, content) in files {
            runner.add_file(filename, content);
        }
        runner
    }

    pub fn add_file(&mut self, filename: &str, content: &str) -> &mut Self {
        let file_path = self.temp_dir.path().join(filename);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }

        fs::write(&file_path, content).expect("Failed to write test file");
        self
    }

    #[must_use]
    pub fn run_test(&self) -> String {
        let temp_path = self.temp_dir.path();
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
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

mod rules;
