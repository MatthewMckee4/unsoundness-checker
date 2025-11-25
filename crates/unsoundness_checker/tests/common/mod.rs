use std::{fmt::Write, fs, path::PathBuf, process::Command, sync::OnceLock};

use ruff_db::{
    diagnostic::{DisplayDiagnosticConfig, DisplayDiagnostics},
    system::{OsSystem, SystemPathBuf},
};
use tempfile::TempDir;
use ty_project::{
    Db, ProjectDatabase, ProjectMetadata, metadata::options::ProjectOptionsOverrides,
};
use unsoundness_checker::{
    check_file, default_rule_registry,
    rule::{Level, RuleSelection},
};

static DISPLAY_CONFIG: OnceLock<DisplayDiagnosticConfig> = OnceLock::new();
static PROJECT_OPTIONS: OnceLock<ProjectOptionsOverrides> = OnceLock::new();

/// Test runner for executing unsoundness checker tests in isolated environments
pub struct TestRunner {
    temp_dir: TempDir,
}

impl TestRunner {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        Self { temp_dir }
    }

    pub const fn temp_dir(&self) -> &TempDir {
        &self.temp_dir
    }

    pub fn venv_dir(&self) -> PathBuf {
        self.temp_dir.path().join(".venv")
    }

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

    /// Configures rules in a pyproject.toml file
    ///
    /// # Arguments
    /// * `rules` - Iterator of tuples containing (`rule_name`, severity)
    ///
    /// # Example
    /// ```
    /// runner.with_rules([
    ///     ("typing-any-used", "error"),
    ///     ("invalid-overload-implementation", "warn"),
    /// ].into_iter());
    /// ```
    pub fn with_rules<I, S1, S2>(&mut self, rules: I) -> &mut Self
    where
        I: Iterator<Item = (S1, S2)>,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut content = String::from("[tool.unsoundness-checker.rules]\n");

        for (rule_name, severity) in rules {
            writeln!(
                &mut content,
                "{} = \"{}\"",
                rule_name.as_ref(),
                severity.as_ref()
            )
            .expect("Failed to write rule");
        }

        self.add_file("pyproject.toml", &content);
        self
    }

    /// Runs the unsoundness checker on all files in the test directory
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

    pub fn run_ty(&self) -> String {
        self.run_external_tool("ty", &["ty", "check"])
    }

    pub fn run_mypy(&self) -> String {
        self.run_external_tool("mypy", &["mypy"])
    }

    pub fn run_pyright(&self) -> String {
        self.run_external_tool("pyright", &["pyright"])
    }

    pub fn run_pyrefly(&self) -> String {
        self.run_external_tool("pyrefly", &["pyrefly", "check"])
    }

    fn run_external_tool(&self, tool: &str, check_command: &[&str]) -> String {
        // Download the tool if it's not already installed
        Command::new("uvx")
            .arg(tool)
            .arg("--help")
            .output()
            .expect("Failed to run external tool");

        let output = Command::new("uvx")
            .args(check_command)
            .arg(self.temp_dir.path())
            .current_dir(self.temp_dir.path())
            .output()
            .expect("Failed to run external tool");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        format!("{stdout}{stderr}")
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub content: String,
    pub language: String,
    pub name: Option<String>,
    pub heading_level: usize,
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
    pub fn from_markdown_content(content: &str) -> Self {
        let parser = Parser::new(content);
        let mut snippets = Vec::new();
        let mut current_code_block: Option<String> = None;
        let mut current_language = String::new();
        let mut current_heading = String::new();
        let mut current_heading_level: usize = 2;
        let mut reading_heading = false;
        let mut heading_counter: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    reading_heading = true;
                    current_heading.clear();
                    current_heading_level = level as usize;
                }
                Event::End(TagEnd::Heading { .. }) => {
                    reading_heading = false;
                }
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
                        let heading_name = if current_heading.is_empty() {
                            "unnamed".to_string()
                        } else {
                            current_heading.clone()
                        };

                        // Track how many snippets we've seen for this heading
                        let counter = heading_counter.entry(heading_name.clone()).or_insert(0);
                        *counter += 1;

                        let snippet_name = if *counter == 1 {
                            heading_name
                        } else {
                            format!("{heading_name} ({counter})")
                        };

                        snippets.push(CodeSnippet {
                            content: code,
                            language: current_language.clone(),
                            name: Some(snippet_name),
                            heading_level: current_heading_level,
                        });
                    }
                }
                Event::Text(text) => {
                    if reading_heading {
                        current_heading.push_str(&text);
                    } else if let Some(ref mut code) = current_code_block {
                        code.push_str(&text);
                    }
                }
                Event::SoftBreak | Event::HardBreak => {
                    if reading_heading {
                        current_heading.push(' ');
                    } else if let Some(ref mut code) = current_code_block {
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
const RESOURCE_DIR_EXTENSIVE: &str = "resources/extensive";

pub fn run_rule_tests(rule_name: &str) -> Vec<(PathBuf, String, String, usize)> {
    let file_path = format!("{RESOURCE_DIR}/{rule_name}.md");
    let rule_tests = RuleTestFile::from_markdown_file(file_path);

    let mut results = Vec::new();

    let rule_registry = default_rule_registry();

    let rule_name_kebab = kebab_case(rule_name);

    let rule_levels = rule_registry
        .rules()
        .iter()
        .map(|rule| {
            if rule.name.to_string() == rule_name_kebab {
                (rule_name_kebab.clone(), Level::Error.to_string())
            } else {
                (rule.name.to_string(), Level::Ignore.to_string())
            }
        })
        .collect::<Vec<_>>();

    // Only run external type checkers if explicitly enabled
    let run_external = std::env::var("UNSOUNDNESS_TEST_EXTERNAL")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    for (idx, snippet) in rule_tests.python_snippets().enumerate() {
        let snippet_name = format!("snippet_{:02}", idx + 1);
        let test_name = snippet.name.as_deref().unwrap_or("unnamed");
        let filename = format!("{test_name}.py");
        let heading_level = snippet.heading_level;

        let mut test_runner = TestRunner::from_file(&filename, &snippet.content);

        test_runner.with_rules(rule_levels.clone().into_iter());

        let temp_path = test_runner.temp_dir().path().to_owned();

        let output = test_runner.run_test();
        results.push((
            temp_path.clone(),
            snippet_name.clone(),
            output,
            heading_level,
        ));

        if cfg!(unix) && run_external && rule_name != "type_checking_directive_used" {
            let ty_output = test_runner.run_ty();
            results.push((
                temp_path.clone(),
                format!("{snippet_name}_ty"),
                ty_output,
                heading_level,
            ));

            let mypy_output = test_runner.run_mypy();
            results.push((
                temp_path.clone(),
                format!("{snippet_name}_mypy"),
                mypy_output,
                heading_level,
            ));

            let pyright_output = test_runner.run_pyright();
            results.push((
                temp_path.clone(),
                format!("{snippet_name}_pyright"),
                pyright_output,
                heading_level,
            ));
        }
    }

    results
}

pub fn run_rule_tests_extensive(rule_name: &str) -> Vec<(PathBuf, String, String, usize)> {
    let file_path = format!("{RESOURCE_DIR_EXTENSIVE}/{rule_name}.md");
    let rule_tests = RuleTestFile::from_markdown_file(file_path);

    let mut results = Vec::new();

    let rule_registry = default_rule_registry();

    let rule_name_kebab = kebab_case(rule_name);

    let rule_levels = rule_registry
        .rules()
        .iter()
        .map(|rule| {
            if rule.name.to_string() == rule_name_kebab {
                (rule_name_kebab.clone(), Level::Error.to_string())
            } else {
                (rule.name.to_string(), Level::Ignore.to_string())
            }
        })
        .collect::<Vec<_>>();

    for snippet in rule_tests.python_snippets() {
        let test_name = snippet.name.as_deref().unwrap_or("unnamed");
        let filename = format!("{test_name}.py");
        let heading_level = snippet.heading_level;

        let mut test_runner = TestRunner::from_file(&filename, &snippet.content);

        test_runner.with_rules(rule_levels.clone().into_iter());

        let temp_path = test_runner.temp_dir().path().to_owned();

        let output = test_runner.run_test();
        results.push((
            temp_path.clone(),
            test_name.to_string(),
            output,
            heading_level,
        ));

        // For extensive tests, we don't run external type checkers
    }

    results
}

/// Converts `snake_case` to `kebab-case`.
fn kebab_case(input: &str) -> String {
    input.replace('_', "-")
}
