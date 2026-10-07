use crate::core::settings::SettingsLoadState;
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
    settings_load_state: SettingsLoadState,
    storage_status: &'static str,
    registry_status: &'static str,
    total_modules: usize,
    enabled_modules: usize,
    allowed_permissions: usize,
}
impl Diagnostics {
    // An explicit allowlist, independent of optional path reveal. No file contents
    // or user-controlled metadata are rendered into the shareable report.
    pub fn support_report(&self) -> String {
        format!(
            "VSA CORE local support report\nVersion: {}\nPlatform: {}\nArchitecture: {}\nRuntime: {}\nBuild: {}\nLocal first: {}\nAccount required: {}\nTelemetry implemented: {}\nSettings: {}\nModules: {} total, {} enabled\nAllowed declared permissions: {}\nModule execution: not implemented\nOS sandbox: not implemented\nPaths and settings contents: excluded\nWrite access: not verified\n",
            self.version, self.platform, self.architecture, self.runtime, self.build_mode,
            self.local_first, self.account_required, self.telemetry_implemented,
            self.storage_status, self.total_modules, self.enabled_modules, self.allowed_permissions,
        )
    }
    pub fn current(
        registry: &ModuleRegistry,
        config_directory: &std::path::Path,
        data_directory: &std::path::Path,
        settings_load_state: SettingsLoadState,
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
            settings_load_state,
            storage_status: match settings_load_state {
                SettingsLoadState::Missing => {
                    "Settings missing; defaults in use; write access not verified"
                }
                SettingsLoadState::Loaded => {
                    "Settings read successfully; write access not verified"
                }
                SettingsLoadState::Invalid => "Invalid settings; existing configuration preserved",
                SettingsLoadState::Unavailable => {
                    "Settings unavailable or unsafe; write access not verified"
                }
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
    fn support_report_is_deterministic_and_never_includes_revealed_paths() {
        let registry = ModuleRegistry::roadmap().unwrap();
        let private = std::path::Path::new("private-user-Żółć-token-fixture");
        for state in [
            SettingsLoadState::Missing,
            SettingsLoadState::Loaded,
            SettingsLoadState::Invalid,
            SettingsLoadState::Unavailable,
        ] {
            let hidden = Diagnostics::current(&registry, private, private, state, false);
            let revealed = Diagnostics::current(&registry, private, private, state, true);
            let report = revealed.support_report();
            assert_eq!(report, hidden.support_report());
            assert_eq!(report, revealed.support_report());
            assert!(!report.contains("private-user"));
            assert!(!report.contains("token-fixture"));
            assert!(report.contains("Modules: 4 total, 0 enabled"));
            assert!(report.contains("OS sandbox: not implemented"));
            assert!(report.contains("Paths and settings contents: excluded"));
        }
    }
    #[test]
    fn paths_require_explicit_opt_in_and_storage_errors_stay_generic() {
        let registry = ModuleRegistry::roadmap().unwrap();
        let path = std::path::Path::new("private-user-directory");
        let value = serde_json::to_value(Diagnostics::current(
            &registry,
            path,
            path,
            SettingsLoadState::Invalid,
            true,
        ))
        .unwrap();
        assert_eq!(value["configDirectory"], "private-user-directory");
        assert_eq!(value["dataDirectory"], "private-user-directory");
        assert_eq!(
            value["storageStatus"],
            "Invalid settings; existing configuration preserved"
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
            SettingsLoadState::Missing,
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
