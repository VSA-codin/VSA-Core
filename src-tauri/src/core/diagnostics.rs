use crate::core::ModuleRegistry;
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
    config_directory: Option<String>,
    data_directory: Option<String>,
    storage_status: &'static str,
    registry_status: &'static str,
    total_modules: usize,
    enabled_modules: usize,
    allowed_permissions: usize,
}
impl Diagnostics {
    pub fn current(
        registry: &ModuleRegistry,
        config_directory: &std::path::Path,
        data_directory: &std::path::Path,
        storage_readable: bool,
        include_paths: bool,
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
            config_directory: include_paths
                .then(|| config_directory.to_string_lossy().into_owned()),
            data_directory: include_paths.then(|| data_directory.to_string_lossy().into_owned()),
            storage_status: if storage_readable {
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
    fn paths_require_explicit_opt_in_and_storage_errors_stay_generic() {
        let registry = ModuleRegistry::roadmap().unwrap();
        let path = std::path::Path::new("private-user-directory");
        let value =
            serde_json::to_value(Diagnostics::current(&registry, path, path, false, true)).unwrap();
        assert_eq!(value["configDirectory"], "private-user-directory");
        assert_eq!(value["dataDirectory"], "private-user-directory");
        assert_eq!(
            value["storageStatus"],
            "Unavailable or invalid; existing configuration preserved"
        );
    }
    #[test]
    fn diagnostic_snapshot_is_limited_and_honest() {
        let directory =
            std::env::temp_dir().join(format!("vsa-diagnostics-missing-{}", std::process::id()));
        let diagnostics = Diagnostics::current(
            &ModuleRegistry::roadmap().unwrap(),
            &directory,
            &directory,
            true,
            false,
        );
        let value = serde_json::to_value(diagnostics).unwrap();
        assert!(value["configDirectory"].is_null());
        assert!(value["dataDirectory"].is_null());
        assert!(!serde_json::to_string(&value)
            .unwrap()
            .contains("vsa-diagnostics-missing"));
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
