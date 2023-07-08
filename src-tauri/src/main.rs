// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setup;
mod terminal;

use setup::setup;
use terminal::Terminal;

fn main() {
    let terminal = Terminal::default();

    tauri::Builder::default()
        .manage(terminal)
        .setup(|app| {
            setup(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
