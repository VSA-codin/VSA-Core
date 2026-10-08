use crate::core::{ModuleDescriptor, ModuleRegistry};
use tauri::State;

#[tauri::command]
pub fn get_modules(registry: State<'_, ModuleRegistry>) -> Result<Vec<ModuleDescriptor>, String> {
    Ok(registry.modules().to_vec())
}
