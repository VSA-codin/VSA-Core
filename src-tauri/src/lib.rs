mod commands;
mod core;
mod security;
mod services;
mod storage;

use commands::{get_core_status, get_modules};
use core::ModuleRegistry;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(
            ModuleRegistry::roadmap().expect("invalid built-in module metadata"),
        ))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_core_status, get_modules])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
