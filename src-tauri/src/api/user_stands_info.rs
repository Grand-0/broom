use crate::view_models::StandInfo;
use tauri::Manager;

#[tauri::command]
pub fn get_user_stands_info(app: tauri::AppHandle) -> Result<Vec<StandInfo>, String> {
    let home_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let config_path = home_dir.join("stands_info.json");

    if !config_path.exists() {}

    let file_content =
        std::fs::read_to_string(&config_path).map_err(|e| format!("Ошибка чтения файла: {}", e))?;

    let config: Vec<StandInfo> =
        serde_json::from_str(&file_content).map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

    Ok(config)
}
