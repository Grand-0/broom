use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

pub fn create_vm(app_data_dir: &Path, name: &str) -> Result<String, String> {
    run_hyper_v_command(app_data_dir, "New-VM", name, &["-Name", name])
}

pub fn remove_vm(app_data_dir: &Path, name: &str) -> Result<String, String> {
    run_hyper_v_command(app_data_dir, "Remove-VM", name, &["-Name", name, "-Force"])
}

pub fn get_vm_status(app_data_dir: &Path, name: &str) -> Result<String, String> {
    run_hyper_v_command(app_data_dir, "Get-VM", name, &["-Name", name])
}

fn log_timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", secs)
}

fn run_hyper_v_command(
    app_data_dir: &Path,
    operation: &str,
    vm_name: &str,
    args: &[&str],
) -> Result<String, String> {
    let logs_dir = app_data_dir.join("logs");
    let _ = fs::create_dir_all(&logs_dir);

    let log_filename = format!("{}_{}_{}.txt", operation, vm_name, log_timestamp());
    let log_path = logs_dir.join(&log_filename);

    let output = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(args.join(" "))
        .output()
        .map_err(|e| format!("Ошибка запуска PowerShell: {}", e))?;

    let mut log_content = String::new();

    if !output.stdout.is_empty() {
        log_content.push_str(&format!(
            "[{}] stdout:\n{}\n",
            log_timestamp(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    if !output.stderr.is_empty() {
        log_content.push_str(&format!(
            "[{}] stderr:\n{}\n",
            log_timestamp(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    log_content.push_str(&format!(
        "[{}] Exit code: {}",
        log_timestamp(),
        output.status.code().unwrap_or(-1)
    ));

    let _ = fs::write(&log_path, log_content);

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(format!(
            "{} для VM {} завершился с ошибкой. Лог: {}",
            operation, vm_name,
            log_path.to_string_lossy()
        ))
    }
}
