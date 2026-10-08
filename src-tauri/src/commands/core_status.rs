use crate::core::{CoreStatus, ModuleRegistry};
use tauri::State;

#[tauri::command]
pub fn get_core_status(registry: State<'_, ModuleRegistry>) -> Result<CoreStatus, String> {
    Ok(CoreStatus::current(
        registry.modules().len(),
        registry.enabled_count(),
    ))
}
