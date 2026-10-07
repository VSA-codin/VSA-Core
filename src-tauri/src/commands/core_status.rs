use crate::core::CoreStatus;

#[tauri::command]
pub fn get_core_status() -> CoreStatus {
    CoreStatus::current()
}
