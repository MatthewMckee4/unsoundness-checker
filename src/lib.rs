pub use crate::diagnostic::register_rules;
use crate::rule::{RuleRegistry, RuleRegistryBuilder};

pub mod checker;
pub mod cli;
pub(crate) mod diagnostic;
pub(crate) mod rule;
pub(crate) mod version;

pub(crate) const NAME: &str = "Unsoundness Checker";

/// Returns the default registry with all known semantic rules.
pub(crate) fn default_rule_registry() -> &'static RuleRegistry {
    static REGISTRY: std::sync::LazyLock<RuleRegistry> = std::sync::LazyLock::new(|| {
        let mut registry = RuleRegistryBuilder::default();
        register_rules(&mut registry);
        registry.build()
    });

    &REGISTRY
}
