// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod explorer;

use explorer::Directory;
use std::sync::Mutex;

struct AppState(Mutex<Directory>);

#[tauri::command]
fn get_curr_directory(state: tauri::State<AppState>) -> Directory {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn go_up(state: tauri::State<AppState>) -> Directory {
    state.0.lock().unwrap().go_up()
}

#[tauri::command]
fn go_down(state: tauri::State<AppState>, file: &str) -> Directory {
    state.0.lock().unwrap().go_down_to(file)
}

#[tauri::command]
fn read_file(state: tauri::State<AppState>, file: &str) -> Option<String> {
    let dir = &state.0.lock().unwrap().files;
    match dir
        .into_iter()
        .find_map(|ent| (ent.name == file).then(|| ent.get_content()))
    {
        Some(s) => s,
        None    => None,
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(Directory::new())))
        .invoke_handler(tauri::generate_handler![
            get_curr_directory,
            go_up,
            go_down,
            read_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
