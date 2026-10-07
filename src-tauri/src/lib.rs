mod commands;
mod core;
mod security;
mod services;
mod storage;

/// Experimental metadata contracts; no module execution or stable binary ABI.
pub mod sdk {
    pub use crate::core::automation::{AutomationPlan, AutomationTrigger};
    pub use crate::core::module_manifest::ModuleManifest;
    pub use crate::core::module_registry::{ModuleDescriptor, ModuleLifecycle};
    pub use crate::security::{Permission, PermissionDecision, PermissionPolicy};
}

use commands::{
    get_core_status, get_diagnostics, get_modules, get_settings, get_support_report,
    recover_settings, settings_recovery_available, update_settings,
};
use core::ModuleRegistry;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(ModuleRegistry::roadmap().map_err(std::io::Error::other)?);
            let directory = app.path().app_config_dir()?;
            app.manage(Mutex::new(services::SettingsService::new(
                storage::SettingsStore::new(directory),
            )));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_core_status,
            get_modules,
            get_settings,
            update_settings,
            settings_recovery_available,
            recover_settings,
            get_diagnostics,
            get_support_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
