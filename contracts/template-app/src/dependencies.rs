use abstract_sdk::os::objects::dependency::StaticDependency;
use abstract_sdk::os::EXCHANGE;

/// An example dependency on the dex contract with version 0.3.0 or higher
const DEX_DEP: StaticDependency = StaticDependency::new(EXCHANGE, &[">=0.3.0"]);

/// Dependencies for the app
pub const TEMPLATE_DEPS: &[StaticDependency] = &[DEX_DEP];

#[cfg(test)]
mod tests {
    use super::*;
    use semver::Comparator;

    #[test]
    fn test_dependencies() {
        TEMPLATE_DEPS.iter().for_each(|dep| {
            dep.check().unwrap();
        });
    }
}
