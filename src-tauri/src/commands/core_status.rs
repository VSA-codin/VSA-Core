use crate::core::{CoreStatus, ModuleRegistry};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_core_status(registry: State<'_, Mutex<ModuleRegistry>>) -> Result<CoreStatus, String> {
    let registry = registry
        .lock()
        .map_err(|_| "Module registry lock is unavailable".to_string())?;

    Ok(CoreStatus::current(
        registry.modules().len(),
        registry.enabled_count(),
    ))
}
