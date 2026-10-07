mod commands;
mod core;
mod security;
mod services;
mod storage;

use commands::{get_core_status, get_diagnostics, get_modules, get_settings, update_settings};
use core::ModuleRegistry;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(
            ModuleRegistry::roadmap().expect("invalid built-in module metadata"),
        ))
        .setup(|app| {
            let directory = app.path().app_config_dir()?;
            app.manage(Mutex::new(services::SettingsService::new(
                storage::SettingsStore::new(directory),
            )));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_core_status,
            get_modules,
            get_settings,
            update_settings,
            get_diagnostics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
