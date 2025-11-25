use std::cell::RefCell;

use ruff_db::{
    diagnostic::{
        Annotation, Diagnostic, DiagnosticId, Severity, Span, SubDiagnostic, SubDiagnosticSeverity,
    },
    files::File,
    parsed::{ParsedModuleRef, parsed_module},
};
use ruff_text_size::{Ranged, TextRange};
use ty_project::Db;

use crate::rule::{RuleId, RuleMetadata, RuleSelection, RuleSource};

/// A type for collecting diagnostics for a given file.
pub(crate) struct Context<'db> {
    db: &'db dyn Db,
    /// The file being checked.
    file: File,
    /// The diagnostics collected so far.
    diagnostics: RefCell<Vec<Diagnostic>>,
    /// The rule selection.
    rule_selection: &'db RuleSelection,
    /// The ast.
    ast: ParsedModuleRef,
}

impl<'db> Context<'db> {
    pub(crate) fn new(db: &'db dyn Db, file: File, rule_selection: &'db RuleSelection) -> Self {
        let ast = parsed_module(db, file).load(db);
        Self {
            db,
            file,
            diagnostics: RefCell::new(Vec::new()),
            rule_selection,
            ast,
        }
    }

    pub(crate) fn report_lint<'ctx, T: Ranged>(
        &'ctx self,
        rule: &'static RuleMetadata,
        ranged: T,
    ) -> Option<LintDiagnosticGuardBuilder<'db, 'ctx>> {
        LintDiagnosticGuardBuilder::new(self, rule, ranged.range())
    }

    pub(crate) const fn rule_selection(&self) -> &RuleSelection {
        self.rule_selection
    }

    pub(crate) const fn file(&self) -> File {
        self.file
    }

    pub(crate) const fn db(&self) -> &'db dyn Db {
        self.db
    }

    pub(crate) fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics.into_inner()
    }

    pub(crate) const fn ast(&self) -> &ParsedModuleRef {
        &self.ast
    }
}

/// A builder for constructing a lint diagnostic guard.
pub(crate) struct LintDiagnosticGuardBuilder<'db, 'ctx> {
    ctx: &'ctx Context<'db>,
    id: DiagnosticId,
    severity: Severity,
    source: RuleSource,
    primary_span: Span,
}

impl<'db, 'ctx> LintDiagnosticGuardBuilder<'db, 'ctx> {
    fn new(ctx: &'ctx Context<'db>, rule: &'static RuleMetadata, range: TextRange) -> Option<Self> {
        let lint_id = RuleId::of(rule);
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
    pub(crate) fn into_diagnostic(
        self,
        message: impl std::fmt::Display,
    ) -> LintDiagnosticGuard<'db, 'ctx> {
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
pub(crate) struct LintDiagnosticGuard<'db, 'ctx> {
    ctx: &'ctx Context<'db>,
    /// The diagnostic that we want to report.
    ///
    /// This is always `Some` until the `Drop` impl.
    diag: Option<Diagnostic>,

    source: RuleSource,
}

impl std::ops::Deref for LintDiagnosticGuard<'_, '_> {
    type Target = Diagnostic;

    fn deref(&self) -> &Diagnostic {
        // OK because `self.diag` is only `None` within `Drop`.
        self.diag.as_ref().unwrap()
    }
}

/// Return a mutable borrow of the diagnostic in this guard.
impl std::ops::DerefMut for LintDiagnosticGuard<'_, '_> {
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
impl Drop for LintDiagnosticGuard<'_, '_> {
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
