use std::{fs, sync::OnceLock};

use ruff_db::{
    diagnostic::{DisplayDiagnosticConfig, DisplayDiagnostics},
    system::{OsSystem, SystemPathBuf},
};
use tempfile::TempDir;
use ty_project::{
    Db, ProjectDatabase, ProjectMetadata, metadata::options::ProjectOptionsOverrides,
};
use unsoundness_checker::{check_file, default_rule_registry, rule::RuleSelection};

static DISPLAY_CONFIG: OnceLock<DisplayDiagnosticConfig> = OnceLock::new();
static PROJECT_OPTIONS: OnceLock<ProjectOptionsOverrides> = OnceLock::new();

/// Test runner for executing unsoundness checker tests in isolated environments
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

    pub fn add_file(&mut self, filename: &str, content: &str) -> &mut Self {
        let file_path = self.temp_dir.path().join(filename);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }

        fs::write(&file_path, content).expect("Failed to write test file");
        self
    }

    /// Runs the unsoundness checker on all files in the test directory
    #[must_use]
    pub fn run_test(&self) -> String {
        Self::run_test_impl(self.temp_dir.path())
    }

    fn run_test_impl(temp_path: &std::path::Path) -> String {
        let cwd =
            SystemPathBuf::from_path_buf(temp_path.to_path_buf()).expect("Failed to convert path");
        let system = OsSystem::new(&cwd);

        let mut project_metadata =
            ProjectMetadata::discover(&cwd, &system).expect("Failed to discover project");
        project_metadata
            .apply_configuration_files(&system)
            .expect("Failed to apply config");

        let rules = project_metadata.options().rules.clone();

        let project_options_overrides =
            PROJECT_OPTIONS.get_or_init(ProjectOptionsOverrides::default);
        project_metadata.apply_overrides(project_options_overrides);

        let db = ProjectDatabase::new(project_metadata, system).expect("Failed to create database");

        let rule_registry = default_rule_registry();
        let (rule_selection, rule_diagnostics) =
            RuleSelection::from_rules_selection(rule_registry, rules.as_ref(), &db);

        let files = db.project().files(&db);
        let mut diagnostics = Vec::new();

        for file in &files {
            diagnostics.extend(check_file(&db, file, &rule_selection));
        }

        let display_config = DISPLAY_CONFIG.get_or_init(DisplayDiagnosticConfig::default);

        let rule_display = DisplayDiagnostics::new(&db, display_config, &rule_diagnostics);

        let display = DisplayDiagnostics::new(&db, display_config, &diagnostics);

        format!("{rule_display}{display}")
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub content: String,
    pub language: String,
    pub name: Option<String>,
}

/// Contains test data parsed from markdown rule files
#[derive(Debug)]
pub struct RuleTestFile {
    pub snippets: Vec<CodeSnippet>,
}

impl RuleTestFile {
    /// Parses a markdown file into a `RuleTestFile`
    pub fn from_markdown_file<P: AsRef<std::path::Path>>(file_path: P) -> Self {
        let content = fs::read_to_string(&file_path).unwrap();

        Self::from_markdown_content(&content)
    }

    /// Parses markdown content into a `RuleTestFile`
    #[must_use]
    pub fn from_markdown_content(content: &str) -> Self {
        let parser = Parser::new(content);
        let mut snippets = Vec::new();
        let mut current_code_block: Option<String> = None;
        let mut current_language = String::new();
        let mut snippet_counter = 0;

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    let lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                        pulldown_cmark::CodeBlockKind::Indented => "text".to_string(),
                    };
                    current_language = lang;
                    current_code_block = Some(String::new());
                }
                Event::End(TagEnd::CodeBlock) => {
                    if let Some(code) = current_code_block.take()
                        && matches!(current_language.as_str(), "python" | "py")
                    {
                        snippet_counter += 1;
                        snippets.push(CodeSnippet {
                            content: code,
                            language: current_language.clone(),
                            name: Some(format!("snippet_{snippet_counter:02}")),
                        });
                    }
                }
                Event::Text(text) => {
                    if let Some(ref mut code) = current_code_block {
                        code.push_str(&text);
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    if let Some(ref mut code) = current_code_block {
                        code.push('\n');
                    }
                }
                _ => {}
            }
        }

        Self { snippets }
    }

    pub fn python_snippets(&self) -> impl Iterator<Item = &CodeSnippet> {
        self.snippets
            .iter()
            .filter(|s| matches!(s.language.as_str(), "python" | "py"))
    }
}

const RESOURCE_DIR: &str = "resources/rules";

#[must_use]
pub fn run_rule_tests(rule_name: &str) -> HashMap<String, String> {
    let file_path = format!("{RESOURCE_DIR}/{rule_name}.md");
    let rule_tests = RuleTestFile::from_markdown_file(file_path);

    let mut results = HashMap::new();

    for snippet in rule_tests.python_snippets() {
        let test_name = snippet.name.as_deref().unwrap_or("unnamed");
        let filename = format!("{test_name}.py");

        let output = TestRunner::from_file(&filename, &snippet.content).run_test();
        results.insert(test_name.to_string(), output);
    }

    results
}
