use super::{trust::TrustState, version::ReleaseVersion};
use crate::{
    core::module_registry::{ModuleDescriptor, ModuleLifecycle},
    security::Permission,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Compatibility {
    CompatibleMetadata,
    CoreTooOld,
    UnsupportedApiMajor,
    UnsupportedExperimentalApiMinor,
    UnspecifiedApi,
    Planned,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogPreview {
    module_id: String,
    compatibility: Compatibility,
    trust: TrustState,
    permission_preview: Vec<Permission>,
    installation_available: bool,
}
impl CatalogPreview {
    /// API comparison is metadata only, and never evidence that execution is safe.
    pub fn new(module: &ModuleDescriptor, core_api: ReleaseVersion) -> Result<Self, &'static str> {
        module.validate()?;
        let compatibility = if module.lifecycle == ModuleLifecycle::Planned {
            Compatibility::Planned
        } else if let Some(required_api) = module.required_core_api {
            let [major, minor, _] = core_api.components();
            let [required_major, required_minor, _] = required_api.components();
            if major != required_major {
                Compatibility::UnsupportedApiMajor
            } else if major == 0 && minor != required_minor {
                Compatibility::UnsupportedExperimentalApiMinor
            } else if core_api < required_api {
                Compatibility::CoreTooOld
            } else {
                Compatibility::CompatibleMetadata
            }
        } else {
            Compatibility::UnspecifiedApi
        };
        Ok(Self {
            module_id: module.id.clone(),
            compatibility,
            trust: TrustState::Unsigned,
            permission_preview: module.declared_permissions.clone(),
            installation_available: false,
        })
    }
}
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CatalogOperation {
    Install,
    Update,
    Remove,
    Rollback,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogPlan {
    module_id: String,
    operation: CatalogOperation,
    blocked_reason: &'static str,
    preserve_user_data: bool,
}
impl CatalogPlan {
    pub fn preview(module_id: String, operation: CatalogOperation) -> Result<Self, &'static str> {
        super::super::module_registry::validate_identifier(&module_id)?;
        Ok(Self {
            module_id,
            operation,
            blocked_reason: "Installation and verification are unavailable",
            preserve_user_data: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compatibility_is_numeric_and_never_authorizes_installation() {
        let core = ReleaseVersion::parse("1.2.0").unwrap();
        let mut m = ModuleDescriptor::new("example", "Example", "Metadata");
        assert_eq!(
            CatalogPreview::new(&m, core).unwrap().compatibility,
            Compatibility::Planned
        );
        m.lifecycle = ModuleLifecycle::Available;
        m.version = Some("1.0.0".into());
        assert_eq!(
            CatalogPreview::new(&m, core).unwrap().compatibility,
            Compatibility::UnspecifiedApi
        );
        for (required, expected) in [
            ("1.1.0", Compatibility::CompatibleMetadata),
            ("1.10.0", Compatibility::CoreTooOld),
            ("2.0.0", Compatibility::UnsupportedApiMajor),
        ] {
            m.required_core_api = Some(ReleaseVersion::parse(required).unwrap());
            let preview = CatalogPreview::new(&m, core).unwrap();
            assert_eq!(preview.compatibility, expected);
            assert_eq!(preview.trust, TrustState::Unsigned);
            assert!(!preview.installation_available);
        }
        m.required_core_api = Some(ReleaseVersion::parse("0.1.0").unwrap());
        assert_eq!(
            CatalogPreview::new(&m, ReleaseVersion::parse("0.2.0").unwrap())
                .unwrap()
                .compatibility,
            Compatibility::UnsupportedExperimentalApiMinor
        );
        m.id = "../escape".into();
        assert!(CatalogPreview::new(&m, core).is_err());
    }
    #[test]
    fn every_catalog_operation_is_a_blocked_preview() {
        for op in [
            CatalogOperation::Install,
            CatalogOperation::Update,
            CatalogOperation::Remove,
            CatalogOperation::Rollback,
        ] {
            let plan = CatalogPlan::preview("example".into(), op).unwrap();
            assert!(plan.preserve_user_data);
            assert!(!plan.blocked_reason.is_empty());
            assert!(CatalogPlan::preview("../escape".into(), op).is_err());
        }
    }
}
