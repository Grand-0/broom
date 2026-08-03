use tauri::Manager;
use tauri_plugin_opener::OpenerExt;

use crate::view_models::{LogInfo, LogType};

#[cfg(windows)]
fn release_foreground() {
    unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::SetForegroundWindow(std::ptr::null_mut());
    }
}

#[cfg(not(windows))]
fn release_foreground() {}

#[tauri::command]
pub fn open_log(app: tauri::AppHandle, log_info: LogInfo) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    let subdir = match log_info.log_type {
        LogType::Session => "operations",
        LogType::Operation => "app",
    };

    let path = app_data_dir
        .join("logs")
        .join(subdir)
        .join(&log_info.log_name);

    if !path.exists() {
        return Err(format!("Лог не найден: {}", path.display()));
    }

    release_foreground();

    app.opener()
        .open_path(path.to_string_lossy().into_owned(), None::<&str>)
        .map_err(|e| format!("Не удалось открыть лог: {}", e))?;

    Ok(())
}
