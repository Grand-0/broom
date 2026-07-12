mod models;

use models::StandInfo;
use std::fs;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_user_stands_info(app: tauri::AppHandle) -> Result<Vec<StandInfo>, String> {
    let home_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let config_path = home_dir.join("stands_info.json");

    if !config_path.exists() {}

    let file_content =
        std::fs::read_to_string(&config_path).map_err(|e| format!("Ошибка чтения файла: {}", e))?;

    let config: Vec<StandInfo> =
        serde_json::from_str(&file_content).map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

    Ok(config)
}

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
                fs::write(&config_path, "{}")?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_user_stands_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
