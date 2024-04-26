mod db;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![some_list, some_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn some_command(app_handle: tauri::AppHandle) -> String {
    db::get_db_path(app_handle)
}

#[tauri::command]
fn some_list<'a>() -> [&'a str; 5] {
    let word_list: [&str; 5] = ["H", "e", "l", "l", "o"];
    word_list
}
