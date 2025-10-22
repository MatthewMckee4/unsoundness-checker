use ruff_python_ast::{Decorator, Expr, StmtReturn};
use ruff_text_size::Ranged;
use ty_python_semantic::types::Type;

use crate::{
    Context, declare_rule,
    rule::{Level, RuleRegistryBuilder, RuleStatus},
};

pub(crate) fn register_rules(registry: &mut RuleRegistryBuilder) {
    registry.register_rule(&TYPING_ANY_USED);
    registry.register_rule(&INVALID_OVERLOAD_IMPLEMENTATION);
    registry.register_rule(&TYPING_OVERLOAD_USED);
    registry.register_rule(&TYPE_CHECKING_DIRECTIVE_USED);
    registry.register_rule(&INVALID_FUNCTION_DEFAULTS);
    registry.register_rule(&SETTING_FUNCTION_CODE_ATTRIBUTE);
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
    /// from typing import Any
    ///
    /// def foo(x: Any) -> Any:
    ///     return x + 1
    ///
    /// foo("1")
    /// ```
    pub (crate) static TYPING_ANY_USED = {
        summary: "detects usage of `typing.Any` in type annotations",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Warn,
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
    /// from typing import overload
    ///
    /// @overload
    /// def foo(x: int) -> str: ...
    /// @overload
    /// def foo(x: str) -> int: ...
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

declare_rule! {
    /// ## What it does
    /// Checks for usage of overloaded functions.
    ///
    /// ## Why is this bad?
    /// Using overloaded functions can lead to runtime errors.
    /// When users don't follow the correct overload implementation, it can lead to unexpected behavior.
    ///
    /// ## Examples
    /// ```python
    /// from typing import overload
    ///
    /// @overload
    /// def foo(x: int) -> str: ...
    /// @overload
    /// def foo(x: str) -> int: ...
    /// def foo(x: int | str) -> int | str:
    ///     return x
    /// ```
    pub (crate) static TYPING_OVERLOAD_USED = {
        summary: "detects usage of overloaded functions",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for usage of type checking directives in comments.
    ///
    /// ## Why is this bad?
    /// Type checking directives like `# type: ignore` suppress type checker warnings,
    /// which can hide potential type errors that may lead to runtime failures.
    /// These directives bypass the safety guarantees that type checking provides.
    ///
    /// ## Examples
    /// ```python
    /// # mypy / standard (PEP 484)
    /// x = "string" + 123  # type: ignore
    /// y = foo()  # type: ignore[attr-defined]
    /// ```
    pub (crate) static TYPE_CHECKING_DIRECTIVE_USED = {
        summary: "detects usage of type checking directives in comments",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Warn,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for invalid settings of the `__defaults__` attribute of a function.
    ///
    /// ## Why is this bad?
    /// Modifying the `__defaults__` attribute with types different to the parameters
    /// can lead to runtime type errors.
    ///
    /// ## Examples
    /// ```python
    /// def foo(x: int = 1) -> int:
    ///     return x
    ///
    /// foo.__defaults__ = ("string",)
    /// result = foo()  # Returns "string" but type checker thinks it's int
    /// ```
    pub (crate) static INVALID_FUNCTION_DEFAULTS = {
        summary: "detects invalid setting of the `__defaults__` attribute of a function",
        status: RuleStatus::stable("1.0.0"),
        default_level: Level::Error,
    }
}

declare_rule! {
    /// ## What it does
    /// Checks for setting the `__code__` attribute of a function.
    ///
    /// ## Why is this bad?
    /// Modifying the `__code__` attribute allows runtime modification
    /// of function internals, which can bypass type checking and lead to runtime type errors.
    /// Type checkers cannot analyze or verify operations performed through code objects.
    ///
    /// ## Examples
    /// ```python
    /// def foo(x: int) -> int:
    ///     return 1
    ///
    /// def bar(x: str) -> str:
    ///     return "bar"
    ///
    /// foo.__code__ = bar.__code__
    /// # Now foo will return a string
    /// ```
    pub (crate) static SETTING_FUNCTION_CODE_ATTRIBUTE = {
        summary: "detects setting the `__code__` attribute of a function",
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

pub(crate) fn report_typing_overload_used(context: &Context, decorator: &Decorator) {
    let Some(builder) = context.report_lint(&TYPING_OVERLOAD_USED, decorator.range()) else {
        return;
    };

    builder.into_diagnostic("Using `typing.overload` can lead to runtime errors.");
}

pub(crate) fn report_type_checking_directive_used(
    context: &Context,
    range: ruff_text_size::TextRange,
    directive: &str,
) {
    let Some(builder) = context.report_lint(&TYPE_CHECKING_DIRECTIVE_USED, range) else {
        return;
    };

    builder.into_diagnostic(format!(
        "Type checking directive `{directive}` suppresses type checker warnings, which may hide potential type errors.",
    ));
}

pub(crate) fn report_setting_function_defaults_attribute(
    context: &Context,
    expr: &Expr,
    new_defaults: &Type,
) {
    let Some(builder) = context.report_lint(&INVALID_FUNCTION_DEFAULTS, expr.range()) else {
        return;
    };

    builder.into_diagnostic(format!(
        "Setting `__defaults__` to an object of type `{}` on a function may lead to runtime type errors.",
        new_defaults.display(context.db())
    ));
}

pub(crate) fn report_setting_function_code_attribute(context: &Context, expr: &Expr) {
    let Some(builder) = context.report_lint(&SETTING_FUNCTION_CODE_ATTRIBUTE, expr.range()) else {
        return;
    };

    builder.into_diagnostic(
        "Setting `__code__` attribute on a function may lead to runtime type errors.",
    );
}
