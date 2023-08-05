// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod explorer;

use explorer::Dir;
use std::sync::Mutex;

struct AppState(Mutex<Dir>);

#[tauri::command]
fn get_curr_directory(state: tauri::State<AppState>) -> Dir {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn go_up(state: tauri::State<AppState>) -> Dir {
    state.0.lock().unwrap().go_up()
}

#[tauri::command]
fn go_down(state: tauri::State<AppState>, file: &str) -> Dir {
    state.0.lock().unwrap().go_down_to(file)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(Dir::new())))
        .invoke_handler(tauri::generate_handler![get_curr_directory, go_up, go_down])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
