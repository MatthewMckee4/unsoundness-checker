pub(crate) use crate::context::Context;
use crate::rule::{RuleRegistry, RuleRegistryBuilder};
pub use crate::rules::register_rules;

pub mod checker;
pub mod cli;
pub(crate) mod context;
pub(crate) mod rule;
pub(crate) mod rules;
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
