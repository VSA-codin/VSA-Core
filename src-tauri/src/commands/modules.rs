use crate::core::{ModuleDescriptor, ModuleRegistry};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn get_modules(
    registry: State<'_, Mutex<ModuleRegistry>>,
) -> Result<Vec<ModuleDescriptor>, String> {
    let registry = registry
        .lock()
        .map_err(|_| "Module registry lock is unavailable".to_string())?;

    Ok(registry.modules().to_vec())
}
