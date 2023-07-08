use crate::terminal::Terminal;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn spawn(app_handle: AppHandle, state: State<Terminal>) {
    println!("Spawn");

    // TODO: remove unwrap
    state.spawn(app_handle, 80, 24, "zsh").unwrap();
}

#[tauri::command]
pub fn write(state: State<Terminal>, data: String) {
    // TODO: remove unwrap
    state.write(data).unwrap();
}

#[tauri::command]
pub fn resize(state: State<Terminal>, rows: u16, cols: u16) {
    // TODO: remove unwrap
    state.resize(cols, rows).unwrap();
}

#[tauri::command]
pub fn dispose(state: State<Terminal>) {
    println!("Dispose");

    // TODO: remove unwrap
    state.dispose().unwrap();
}

#[tauri::command]
pub fn open(uri: String) {
    println!("Open URI: {}", uri);

    // TODO: remove unwrap
    open::that(uri).unwrap();
}
