//! Allowlist-first future bundle manifest; no attachments or export capability.
use crate::core::{diagnostics::Diagnostics, settings::SettingsLoadState};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportManifest {
    schema_version: u32,
    version: &'static str,
    platform: &'static str,
    architecture: &'static str,
    build_mode: &'static str,
    settings_state: SettingsLoadState,
    module_count: usize,
    enabled_module_count: usize,
    module_execution_available: bool,
    vault_available: bool,
    updater_available: bool,
    network_backend_available: bool,
    attachments: [(); 0],
}
impl SupportManifest {
    pub(crate) fn from_diagnostics(d: &Diagnostics) -> Self {
        Self {
            schema_version: 1,
            version: d.version,
            platform: d.platform,
            architecture: d.architecture,
            build_mode: d.build_mode,
            settings_state: d.settings_load_state,
            module_count: d.total_modules,
            enabled_module_count: d.enabled_modules,
            module_execution_available: false,
            vault_available: false,
            updater_available: false,
            network_backend_available: false,
            attachments: [],
        }
    }
}
