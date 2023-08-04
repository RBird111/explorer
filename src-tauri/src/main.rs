// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod explorer;

use explorer::Dir;

#[tauri::command]
fn get_curr_directory() -> Dir {
    Dir::new()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_curr_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
