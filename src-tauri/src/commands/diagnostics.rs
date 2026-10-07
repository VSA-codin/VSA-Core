use crate::{
    core::{diagnostics::Diagnostics, ModuleRegistry},
    services::SettingsService,
};
use std::sync::Mutex;
use tauri::{Manager, State};

#[tauri::command]
pub fn get_diagnostics(
    app: tauri::AppHandle,
    include_paths: Option<bool>,
    registry: State<'_, ModuleRegistry>,
    settings: State<'_, Mutex<SettingsService>>,
) -> Result<Diagnostics, String> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|_| "Application data location unavailable")?;
    let settings = settings
        .lock()
        .map_err(|_| "Settings service unavailable")?;
    Ok(settings.diagnostics(&registry, &directory, include_paths.unwrap_or(false)))
}
