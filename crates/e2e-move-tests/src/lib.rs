pub mod harness;
pub mod test_utils;

// use anyhow::bail;
pub use harness::*;
// use move_package::package_hooks::PackageHooks;
// use move_package::source_package::parsed_manifest::CustomDepInfo;
// use move_symbol_pool::Symbol;

#[cfg(test)]
mod tests;

// pub(crate) struct InitiaPackageHooks {}
// pub const UPGRADE_POLICY_CUSTOM_FIELD: &str = "upgrade_policy";

// impl PackageHooks for InitiaPackageHooks {
//     fn custom_package_info_fields(&self) -> Vec<String> {
//         vec![UPGRADE_POLICY_CUSTOM_FIELD.to_string()]
//     }

//     fn custom_dependency_key(&self) -> Option<String> {
//         Some("initia".to_string())
//     }

//     fn resolve_custom_dependency(
//         &self,
//         _dep_name: Symbol,
//         _info: &CustomDepInfo,
//     ) -> anyhow::Result<()> {
//         bail!("not used")
//     }
// }
