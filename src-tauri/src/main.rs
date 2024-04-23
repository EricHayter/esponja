// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// creating a custom function that can be invoked by the javascript frontend
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
