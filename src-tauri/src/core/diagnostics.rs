use crate::{core::ModuleRegistry, services::SettingsService};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostics {
    version: &'static str,
    platform: &'static str,
    architecture: &'static str,
    runtime: &'static str,
    build_mode: &'static str,
    local_first: bool,
    telemetry_implemented: bool,
    account_required: bool,
    config_directory: String,
    data_directory: String,
    storage_status: &'static str,
    registry_status: &'static str,
    total_modules: usize,
    enabled_modules: usize,
    allowed_permissions: usize,
}
impl Diagnostics {
    pub fn current(
        registry: &ModuleRegistry,
        settings: &SettingsService,
        data_directory: &std::path::Path,
    ) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            platform: std::env::consts::OS,
            architecture: std::env::consts::ARCH,
            runtime: "Tauri 2 + Rust",
            build_mode: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            },
            local_first: true,
            telemetry_implemented: false,
            account_required: false,
            config_directory: settings.directory().to_string_lossy().into_owned(),
            data_directory: data_directory.to_string_lossy().into_owned(),
            storage_status: if settings.get().is_ok() {
                "Readable (defaults if missing); write access not verified"
            } else {
                "Unavailable or invalid; existing configuration preserved"
            },
            registry_status: "Metadata registry available; execution not implemented",
            total_modules: registry.modules().len(),
            enabled_modules: registry.enabled_count(),
            allowed_permissions: registry.allowed_permission_count(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diagnostic_snapshot_is_limited_and_honest() {
        let directory =
            std::env::temp_dir().join(format!("vsa-diagnostics-missing-{}", std::process::id()));
        let service = SettingsService::new(crate::storage::SettingsStore::new(directory.clone()));
        let diagnostics =
            Diagnostics::current(&ModuleRegistry::roadmap().unwrap(), &service, &directory);
        let value = serde_json::to_value(diagnostics).unwrap();
        assert_eq!(value["totalModules"], 4);
        assert_eq!(value["enabledModules"], 0);
        assert_eq!(value["allowedPermissions"], 0);
        assert_eq!(value["telemetryImplemented"], false);
        assert_eq!(value["accountRequired"], false);
        for excluded in [
            "environment",
            "password",
            "token",
            "cookies",
            "browserHistory",
        ] {
            assert!(value.get(excluded).is_none());
        }
    }
}
