use std::fs;
use std::path::Path;

use crate::view_models::StandInfo;

fn stands_file_path(app_data_dir: &Path) -> std::path::PathBuf {
    app_data_dir.join("stands_info.json")
}

pub fn load_stands(app_data_dir: &Path) -> Result<Vec<StandInfo>, String> {
    let path = stands_file_path(app_data_dir);

    if !path.exists() {
        return Ok(vec![]);
    }

    let file_content =
        fs::read_to_string(&path).map_err(|e| format!("Ошибка чтения файла: {}", e))?;

    let stands: Vec<StandInfo> =
        serde_json::from_str(&file_content).map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

    Ok(stands)
}

pub fn save_stands(app_data_dir: &Path, stands: &[StandInfo]) -> Result<(), String> {
    let path = stands_file_path(app_data_dir);

    let json = serde_json::to_string_pretty(stands)
        .map_err(|e| format!("Ошибка сериализации JSON: {}", e))?;

    fs::write(&path, json).map_err(|e| format!("Ошибка записи файла: {}", e))?;

    Ok(())
}

pub fn add_stand(app_data_dir: &Path, stand: StandInfo) -> Result<(), String> {
    let mut stands = load_stands(app_data_dir)?;
    stands.push(stand);
    save_stands(app_data_dir, &stands)
}

pub fn remove_stand(app_data_dir: &Path, web_server_name: &str) -> Result<(), String> {
    let stands = load_stands(app_data_dir)?;
    let filtered: Vec<StandInfo> = stands
        .into_iter()
        .filter(|s| s.web_server_name != web_server_name)
        .collect();
    save_stands(app_data_dir, &filtered)
}

pub fn update_stand(
    app_data_dir: &Path,
    web_server_name: &str,
    updated: StandInfo,
) -> Result<(), String> {
    let mut stands = load_stands(app_data_dir)?;
    if let Some(stand) = stands
        .iter_mut()
        .find(|s| s.web_server_name == web_server_name)
    {
        *stand = updated;
        save_stands(app_data_dir, &stands)
    } else {
        Err(format!(
            "Стенд с именем {} не найден",
            web_server_name
        ))
    }
}
