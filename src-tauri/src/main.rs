// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod terminal;

use tauri::{AppHandle, State};
use terminal::Terminal;

#[tauri::command]
fn spawn(app_handle: AppHandle, state: State<Terminal>) {
    println!("Spawn");

    state.spawn(app_handle, 80, 24, "zsh").unwrap();
}

#[tauri::command]
fn write(state: State<Terminal>, data: String) {
    state.write(data).unwrap();
}

#[tauri::command]
fn resize(state: State<Terminal>, rows: u16, cols: u16) {
    state.resize(cols, rows).unwrap();
}

#[tauri::command]
fn dispose(state: State<Terminal>) {
    println!("Dispose");

    state.dispose().unwrap();
}

fn main() {
    let terminal = Terminal::new();

    tauri::Builder::default()
        .manage(terminal)
        .invoke_handler(tauri::generate_handler![spawn, write, resize, dispose])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
