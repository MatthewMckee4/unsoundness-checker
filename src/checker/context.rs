use std::cell::RefCell;

use ruff_db::{
    diagnostic::{
        Annotation, Diagnostic, DiagnosticId, DiagnosticTag, IntoDiagnosticMessage, Severity, Span,
        SubDiagnostic, SubDiagnosticSeverity,
    },
    files::File,
};
use ruff_text_size::{Ranged, TextRange};

use crate::rule::{RuleId, RuleMetadata, RuleRegistry, RuleSelection, RuleSource};

/// A type for collecting diagnostics for a given file.
pub struct Context {
    /// The file being checked.
    file: File,
    /// The rule registry.
    rule_registry: RuleRegistry,
    /// The diagnostics collected so far.
    diagnostics: RefCell<Vec<Diagnostic>>,
    /// The rule selection.
    rule_selection: RuleSelection,
}

impl Context {
    pub fn new(file: File, rule_registry: RuleRegistry, rule_selection: RuleSelection) -> Self {
        Self {
            file,
            rule_registry,
            diagnostics: RefCell::new(Vec::new()),
            rule_selection,
        }
    }

    pub fn report_lint<'ctx, T: Ranged>(
        &'ctx self,
        rule: &'static RuleMetadata,
        ranged: T,
    ) -> Option<LintDiagnosticGuardBuilder<'ctx>> {
        LintDiagnosticGuardBuilder::new(self, rule, ranged.range())
    }

    pub fn rule_selection(&self) -> &RuleSelection {
        &self.rule_selection
    }

    pub fn file(&self) -> File {
        self.file
    }
}

/// A builder for constructing a lint diagnostic guard.
///
/// This type exists to separate the phases of "check if a diagnostic should
/// be reported" and "build the actual diagnostic." It's why, for example,
/// `InferContext::report_lint` only requires a `LintMetadata` (and a range),
/// but this builder further requires a message before one can mutate the
/// diagnostic. This is because the `LintMetadata` can be used to derive
/// the diagnostic ID and its severity (based on configuration). Combined
/// with a message you get the minimum amount of data required to build a
/// `Diagnostic`.
///
/// Additionally, the range is used to construct a primary annotation (without
/// a message) using the file current being type checked. The range given to
/// `InferContext::report_lint` must be from the file currently being type
/// checked.
///
/// If callers need to report a diagnostic with an identifier type other
/// than `DiagnosticId::Lint`, then they should use the more general
/// `InferContext::report_diagnostic` API. But note that this API will not take
/// rule selection or suppressions into account.
///
/// # When is the diagnostic added?
///
/// When a builder is not returned by `InferContext::report_lint`, then
/// it is known that the diagnostic should not be reported. This can happen
/// when the diagnostic is disabled or suppressed (among other reasons).
pub struct LintDiagnosticGuardBuilder<'ctx> {
    ctx: &'ctx Context,
    id: DiagnosticId,
    severity: Severity,
    source: RuleSource,
    primary_span: Span,
}

impl<'ctx> LintDiagnosticGuardBuilder<'ctx> {
    fn new(
        ctx: &'ctx Context,
        rule: &'static RuleMetadata,
        range: TextRange,
    ) -> Option<LintDiagnosticGuardBuilder<'ctx>> {
        let lint_id = RuleId::of(rule);
        // Skip over diagnostics if the rule is disabled.
        let (severity, source) = ctx.rule_selection().get(lint_id)?;

        let id = DiagnosticId::Lint(rule.name());

        let primary_span = Span::from(ctx.file()).with_range(range);
        Some(LintDiagnosticGuardBuilder {
            ctx,
            id,
            severity,
            source,
            primary_span,
        })
    }

    /// Create a new lint diagnostic guard.
    ///
    /// This initializes a new diagnostic using the given message along with
    /// the ID and severity derived from the `LintMetadata` used to create
    /// this builder. The diagnostic also includes a primary annotation
    /// without a message. To add a message to this primary annotation, use
    /// `LintDiagnosticGuard::set_primary_message`.
    ///
    /// The diagnostic can be further mutated on the guard via its `DerefMut`
    /// impl to `Diagnostic`.
    pub fn into_diagnostic(self, message: impl std::fmt::Display) -> LintDiagnosticGuard<'ctx> {
        let mut diag = Diagnostic::new(self.id, self.severity, message);
        // This is why `LintDiagnosticGuard::set_primary_message` exists.
        // We add the primary annotation here (because it's required), but
        // the optional message can be added later. We could accept it here
        // in this `build` method, but we already accept the main diagnostic
        // message. So the messages are likely to be quite confusable.
        diag.annotate(Annotation::primary(self.primary_span.clone()));
        LintDiagnosticGuard {
            ctx: self.ctx,
            source: self.source,
            diag: Some(diag),
        }
    }
}

/// An abstraction for mutating a diagnostic through the lense of a lint.
///
/// Callers can build this guard by starting with `InferContext::report_lint`.
///
/// There are two primary functions of this guard, which mutably derefs to
/// a `Diagnostic`:
///
/// * On `Drop`, the underlying diagnostic is added to the typing context.
/// * Some convenience methods for mutating the underlying `Diagnostic`
///   in lint context. For example, `LintDiagnosticGuard::set_primary_message`
///   will attach a message to the primary span on the diagnostic.
pub struct LintDiagnosticGuard<'ctx> {
    /// The typing context.
    ctx: &'ctx Context,
    /// The diagnostic that we want to report.
    ///
    /// This is always `Some` until the `Drop` impl.
    diag: Option<Diagnostic>,

    source: RuleSource,
}

impl LintDiagnosticGuard<'_> {
    pub fn set_primary_message(&mut self, message: impl IntoDiagnosticMessage) {
        let ann = self.primary_annotation_mut().unwrap();
        ann.set_message(message);
    }

    /// Adds a tag on the primary annotation for this diagnostic.
    ///
    /// This tag is associated with the primary annotation created
    /// for every `Diagnostic` that uses the `LintDiagnosticGuard` API.
    /// Specifically, the annotation is derived from the `TextRange` given to
    /// the `InferContext::report_lint` API.
    ///
    /// Callers can add additional primary or secondary annotations via the
    /// `DerefMut` trait implementation to a `Diagnostic`.
    pub fn add_primary_tag(&mut self, tag: DiagnosticTag) {
        let ann = self.primary_annotation_mut().unwrap();
        ann.push_tag(tag);
    }
}

impl std::ops::Deref for LintDiagnosticGuard<'_> {
    type Target = Diagnostic;

    fn deref(&self) -> &Diagnostic {
        // OK because `self.diag` is only `None` within `Drop`.
        self.diag.as_ref().unwrap()
    }
}

/// Return a mutable borrow of the diagnostic in this guard.
///
/// Callers may mutate the diagnostic to add new sub-diagnostics
/// or annotations.
///
/// The diagnostic is added to the typing context, if appropriate,
/// when this guard is dropped.
impl std::ops::DerefMut for LintDiagnosticGuard<'_> {
    fn deref_mut(&mut self) -> &mut Diagnostic {
        // OK because `self.diag` is only `None` within `Drop`.
        self.diag.as_mut().unwrap()
    }
}

/// Finishes use of this guard.
///
/// This will add the lint as a diagnostic to the typing context if
/// appropriate. The diagnostic may be skipped, for example, if there is a
/// relevant suppression.
impl Drop for LintDiagnosticGuard<'_> {
    fn drop(&mut self) {
        // OK because the only way `self.diag` is `None`
        // is via this impl, which can only run at most
        // once.
        let mut diag = self.diag.take().unwrap();

        diag.sub(SubDiagnostic::new(
            SubDiagnosticSeverity::Info,
            match self.source {
                RuleSource::Default => format!("rule `{}` is enabled by default", diag.id()),
                RuleSource::Cli => format!("rule `{}` was selected on the command line", diag.id()),
                RuleSource::File => {
                    format!(
                        "rule `{}` was selected in the configuration file",
                        diag.id()
                    )
                }
            },
        ));

        self.ctx.diagnostics.borrow_mut().push(diag);
    }
}
