use crate::store::stands_store;
use crate::view_models::StandInfo;
use tauri::Manager;

#[tauri::command]
pub fn get_user_stands_info(app: tauri::AppHandle) -> Result<Vec<StandInfo>, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    stands_store::load_stands(&app_data_dir)
}
