use ruff_python_ast::{Expr, StmtReturn};
use ruff_text_size::Ranged;
use ty_python_semantic::types::Type;

use crate::{
    Context, declare_rule,
    rule::{Level, RuleRegistryBuilder, RuleStatus},
};

/// Registers all known type check lints.
pub(crate) fn register_rules(registry: &mut RuleRegistryBuilder) {
    registry.register_rule(&TYPING_ANY_USED);
    registry.register_rule(&INVALID_OVERLOAD_IMPLEMENTATION);
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
    pub (crate) static TYPING_ANY_USED = {
        summary: "detects usage of `typing.Any` in type annotations",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Error,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for invalid overload implementation.
    ///
    /// ## Why is this bad?
    /// Invalid overload implementation can lead to runtime errors.
    ///
    /// ## Examples
    /// ```python
    /// @overload
    /// def foo(x: int) -> str: ...
    ///
    /// @overload
    /// def foo(x: str) -> int: ...
    ///
    /// def foo(x: int | str) -> int | str:
    ///     return x
    ///
    /// foo("1")
    /// ```
    pub (crate) static INVALID_OVERLOAD_IMPLEMENTATION = {
        summary: "detects invalid overload implementation",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Error,
    }
}

pub(crate) fn report_typing_any_used(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&TYPING_ANY_USED, expr.range()) else {
        return;
    };

    builder.into_diagnostic("Using `typing.Any` in type annotations can lead to runtime errors.");
}

pub(crate) fn report_invalid_overload_implementation<'db>(
    context: &Context<'db>,
    return_statement: &StmtReturn,
    return_type: Option<&'db Type<'db>>,
    overload_return_types: &[Option<Type<'db>>],
) {
    let Some(builder) =
        context.report_lint(&INVALID_OVERLOAD_IMPLEMENTATION, return_statement.range())
    else {
        return;
    };

    let mut diagnostic =
        builder.into_diagnostic("Invalid overload implementation can lead to runtime errors.");

    let get_return_type_display = |ty: Option<&Type<'_>>| {
        format!(
            "`{}`",
            ty.map_or_else(
                || "None".to_string(),
                |ty| ty.display(context.db()).to_string(),
            )
        )
    };

    let return_type_display = get_return_type_display(return_type);

    let overload_return_types_display = overload_return_types
        .iter()
        .map(|ty| get_return_type_display(ty.as_ref()))
        .collect::<Vec<_>>()
        .join(", ");

    diagnostic.info(format_args!(
        "This overload implementation is invalid as {return_type_display} is not \
        assignable to any of the overload return types ({overload_return_types_display})"
    ));
}
