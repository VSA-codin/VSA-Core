use crate::{core::settings::AppSettings, services::SettingsService};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_settings(service: State<'_, Mutex<SettingsService>>) -> Result<AppSettings, String> {
    service
        .lock()
        .map_err(|_| "Settings service unavailable")?
        .get()
        .map_err(str::to_owned)
}
#[tauri::command]
pub fn update_settings(
    settings: AppSettings,
    service: State<'_, Mutex<SettingsService>>,
) -> Result<AppSettings, String> {
    service
        .lock()
        .map_err(|_| "Settings service unavailable")?
        .update(settings)
        .map_err(str::to_owned)
}

#[tauri::command]
pub fn settings_recovery_available(
    service: State<'_, Mutex<SettingsService>>,
) -> Result<bool, String> {
    Ok(service
        .lock()
        .map_err(|_| "Settings service unavailable")?
        .recovery_available())
}
#[tauri::command]
pub fn recover_settings(service: State<'_, Mutex<SettingsService>>) -> Result<AppSettings, String> {
    service
        .lock()
        .map_err(|_| "Settings service unavailable")?
        .recover()
        .map_err(str::to_owned)
}
