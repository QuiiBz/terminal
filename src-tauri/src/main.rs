// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod terminal;

use commands::{dispose, open, resize, spawn, write};
use terminal::Terminal;

fn main() {
    let terminal = Terminal::default();

    tauri::Builder::default()
        .manage(terminal)
        .invoke_handler(tauri::generate_handler![
            spawn, write, resize, dispose, open
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
