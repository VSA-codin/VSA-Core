mod commands;
mod core;
mod security;
mod services;
mod storage;

use commands::get_core_status;

// Temporary command used to verify communication between React and Rust.
// It will be removed once the real VSA CORE commands are in place.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_core_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
