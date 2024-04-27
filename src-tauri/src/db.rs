use std::fs;
use std::path::Path;

pub fn init_db(db_path: &str) {
    if !Path::new(db_path).exists() {
        create_db_file(db_path)
    } 
}

fn create_db_file(db_path: &str) {
    let db_dir = Path::new(db_path).parent().unwrap();

    // if the parent directory does not exist, create it
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // create the database file
    fs::File::create(db_path).unwrap();
}

// use this with a 
pub fn get_db_path(app_handle: &tauri::AppHandle) -> String {
//    let home_dir = dirs::home_dir().unwrap();
//    home_dir.to_str().unwrap().to_string() + 
    let binding = app_handle.path_resolver().app_data_dir().unwrap();
    let app_data_dir = binding.to_str().unwrap();

    app_data_dir.to_string() + "/db.sqlite"
}
