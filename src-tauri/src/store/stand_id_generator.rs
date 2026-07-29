use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StandCounter {
    last_id: u32,
}

fn counter_file_path(app_data_dir: &Path) -> std::path::PathBuf {
    app_data_dir.join("stand_counter.json")
}

pub fn generate_id(app_data_dir: &Path) -> Result<String, String> {
    let path = counter_file_path(app_data_dir);

    let mut counter: StandCounter = if path.exists() {
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Ошибка чтения счётчика: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("Ошибка парсинга счётчика: {}", e))?
    } else {
        StandCounter { last_id: 0 }
    };

    counter.last_id += 1;
    let new_id = counter.last_id;

    let json = serde_json::to_string_pretty(&counter)
        .map_err(|e| format!("Ошибка сериализации счётчика: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Ошибка записи счётчика: {}", e))?;

    Ok(format!("{:03}", new_id))
}
