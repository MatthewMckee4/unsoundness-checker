use ruff_python_ast::{AnyParameterRef, Expr};
use ruff_text_size::Ranged;

use crate::{
    checker::context::Context,
    declare_rule,
    rule::{Level, RuleRegistryBuilder, RuleStatus},
};

/// Registers all known type check lints.
pub(crate) fn register_rules(registry: &mut RuleRegistryBuilder) {
    registry.register_rule(&TYPING_ANY_USED);
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of `typing.Any` in type annotations.
    ///
    /// ## Why is this bad?
    /// Using `typing.Any` in type annotations can lead to runtime errors.
    ///
    /// ## Examples
    /// ```python
    /// def foo(x: Any) -> Any:
    ///     return x + 1
    ///
    /// foo("1")
    /// ```
    pub(crate) static TYPING_ANY_USED = {
        summary: "detects usage of `typing.Any` in type annotations",
        status: RuleStatus::preview("1.0.0"),
        default_level: Level::Error,
    }
}

fn report_typing_any_used(context: Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&TYPING_ANY_USED, expr.range()) else {
        return;
    };

    let mut diagnostic = builder
        .into_diagnostic("Using `typing.Any` in type annotations can lead to runtime errors.");
}
