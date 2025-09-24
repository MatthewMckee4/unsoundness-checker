pub(crate) use crate::diagnostic::register_rules;
use crate::rule::{RuleRegistry, RuleRegistryBuilder};

pub(crate) mod checker;
pub(crate) mod diagnostic;
pub(crate) mod rule;

/// Returns the default registry with all known semantic rules.
pub(crate) fn default_rule_registry() -> &'static RuleRegistry {
    static REGISTRY: std::sync::LazyLock<RuleRegistry> = std::sync::LazyLock::new(|| {
        let mut registry = RuleRegistryBuilder::default();
        register_rules(&mut registry);
        registry.build()
    });

    &REGISTRY
}
