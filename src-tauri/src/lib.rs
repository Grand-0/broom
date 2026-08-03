mod api;
pub mod constants;
mod models;
mod services;
mod store;
mod view_models;

use std::fs;
use tauri::Manager;

use crate::services::{log_cleanup, logging};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path().app_data_dir().map_err(|e| e.to_string())?;

            logging::install_panic_hook();
            logging::init(&path)?;
            log_cleanup::start_daily_cleanup(path.clone());

            let config_path = path.join("stands_info.json");

            if !config_path.exists() {
                if let Some(parent) = config_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&config_path, "[]")?;
            }

            log::info!("broom started");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            api::create_stand::create_stand,
            api::create_stand_form::get_create_stand_form_data,
            api::logs::open_log,
            api::user_stands_info::get_user_stands_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
