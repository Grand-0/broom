use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

pub struct ExecutionResult {
    pub success: bool,
    pub log_path: String,
}

fn log_timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", secs)
}

pub fn execute_script(
    app_data_dir: &Path,
    script_path: &str,
    args: &[&str],
    operation: &str,
    stand_name: &str,
) -> ExecutionResult {
    let logs_dir = app_data_dir.join("logs");
    let _ = fs::create_dir_all(&logs_dir);

    let log_filename = format!("{}_{}_{}.txt", operation, stand_name, log_timestamp());
    let log_path = logs_dir.join(&log_filename);

    let output = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-File")
        .arg(script_path)
        .args(args)
        .output();

    match output {
        Ok(output) => {
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

            ExecutionResult {
                success: output.status.success(),
                log_path: log_path.to_string_lossy().to_string(),
            }
        }
        Err(e) => {
            let log_content = format!(
                "[{}] Failed to start process: {}",
                log_timestamp(),
                e
            );
            let _ = fs::write(&log_path, log_content);

            ExecutionResult {
                success: false,
                log_path: log_path.to_string_lossy().to_string(),
            }
        }
    }
}
