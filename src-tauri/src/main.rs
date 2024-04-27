// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                        some_list,
                        some_command,
                        get_phrases
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod db;

#[tauri::command]
fn some_command(app_handle: tauri::AppHandle) -> String {
    db::get_db_path(&app_handle)
}

#[tauri::command]
fn some_list<'a>() -> [&'a str; 5] {
    let word_list: [&str; 5] = ["H", "e", "l", "l", "o"];
    word_list
}

// make some command to get all entries in the phrase table
#[tauri::command]
fn get_phrases(app_handle: tauri::AppHandle) -> Vec<(String, String)> {
    let path = db::get_db_path(&app_handle);
    db::init_db(&path);

    let connection = sqlite::open(path).unwrap();

    let query = "
        SELECT * FROM phrase;
    ";
    let mut query = connection.prepare(query).unwrap();
    let mut results = Vec::new();
    while let Ok(sqlite::State::Row) = query.next() {
        results.push((
                query.read::<String, _>("english_definition").unwrap(),
                query.read::<String, _>("spanish_definition").unwrap()
            ));
    }
    results
}
