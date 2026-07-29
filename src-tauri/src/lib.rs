mod api;
pub mod constants;
mod models;
mod view_models;

use std::fs;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
            let config_path = path.join("stands_info.json");

            if !config_path.exists() {
                if let Some(parent) = config_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&config_path, "[]")?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            api::create_stand_form::get_create_stand_form_data,
            api::user_stands_info::get_user_stands_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
