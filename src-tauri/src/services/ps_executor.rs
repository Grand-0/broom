use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};

use crate::services::logging;

pub struct ExecutionResult {
    pub success: bool,
    pub log_path: String,
    pub stdout: String,
}

pub fn execute_script(
    app_data_dir: &Path,
    script_path: &str,
    args: &[&str],
    operation: &str,
    stand_name: &str,
    session_log: Option<&Path>,
) -> ExecutionResult {
    let mut command = Command::new("powershell.exe");
    command
        .arg("-NoProfile")
        .arg("-File")
        .arg(script_path)
        .args(args);
    run(app_data_dir, operation, stand_name, command, session_log)
}

pub fn execute_command(
    app_data_dir: &Path,
    ps_command: &str,
    operation: &str,
    stand_name: &str,
    session_log: Option<&Path>,
) -> ExecutionResult {
    let mut command = Command::new("powershell.exe");
    command.arg("-NoProfile").arg("-Command").arg(ps_command);
    run(app_data_dir, operation, stand_name, command, session_log)
}

pub fn execute_technical(
    operation: &str,
    stand_name: &str,
    mut command: Command,
) -> Result<String, String> {
    log::info!(
        "ps start  op={} stand={} (technical)",
        operation,
        stand_name
    );

    let output = command.output().map_err(|e| {
        log::error!("ps failed op={} stand={} err={}", operation, stand_name, e);
        format!("Ошибка запуска PowerShell: {}", e)
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let code = output.status.code().unwrap_or(-1);

    if output.status.success() {
        log::info!("ps exit   op={} stand={} code={}", operation, stand_name, code);
        Ok(stdout)
    } else {
        log::error!(
            "ps exit   op={} stand={} code={} stderr={}",
            operation,
            stand_name,
            code,
            stderr
        );
        Err(format!(
            "{} для {} завершился с ошибкой (код {}). Подробности в техническом логе.",
            operation, stand_name, code
        ))
    }
}

pub fn session_log_path(
    app_data_dir: &Path,
    operation: &str,
    stand_name: &str,
) -> std::path::PathBuf {
    let logs_dir = app_data_dir.join("logs").join("operations");
    let _ = fs::create_dir_all(&logs_dir);
    logs_dir.join(format!(
        "{}_{}_{}.txt",
        operation,
        stand_name,
        logging::file_timestamp()
    ))
}

pub fn append_log_line(log_path: &Path, line: &str) {
    let content = format!("[{}] {}\n", logging::message_timestamp(), line);
    let result = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .and_then(|mut file| file.write_all(content.as_bytes()));
    if let Err(e) = result {
        log::warn!("Не удалось записать лог {}: {}", log_path.display(), e);
    }
}

fn run(
    app_data_dir: &Path,
    operation: &str,
    stand_name: &str,
    mut command: Command,
    session_log: Option<&Path>,
) -> ExecutionResult {
    let log_path = match session_log {
        Some(path) => path.to_path_buf(),
        None => session_log_path(app_data_dir, operation, stand_name),
    };

    log::info!(
        "ps start  op={} stand={} log={}",
        operation,
        stand_name,
        log_path.display()
    );

    let output = command.output();

    match output {
        Ok(output) => {
            let mut content = String::new();
            if session_log.is_some() {
                content.push_str(&format!(
                    "\n[{}] {}\n",
                    logging::message_timestamp(),
                    "=".repeat(60)
                ));
            }
            content.push_str(&build_output_block(operation, stand_name, &output));

            write_log(&log_path, &content, session_log.is_some());

            let success = output.status.success();
            let code = output.status.code().unwrap_or(-1);

            if success {
                log::info!(
                    "ps exit   op={} stand={} code={} log={}",
                    operation,
                    stand_name,
                    code,
                    log_path.display()
                );
            } else {
                log::error!(
                    "ps exit   op={} stand={} code={} log={}",
                    operation,
                    stand_name,
                    code,
                    log_path.display()
                );
            }

            ExecutionResult {
                success,
                log_path: log_path.to_string_lossy().to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            }
        }
        Err(e) => {
            let mut content = String::new();
            if session_log.is_some() {
                content.push_str(&format!(
                    "\n[{}] {}\n",
                    logging::message_timestamp(),
                    "=".repeat(60)
                ));
            }
            content.push_str(&format!(
                "[{}] Step: {} ({})\n[{}] Failed to start process: {}\n",
                logging::message_timestamp(),
                operation,
                stand_name,
                logging::message_timestamp(),
                e
            ));

            write_log(&log_path, &content, session_log.is_some());

            log::error!(
                "ps failed op={} stand={} err={} log={}",
                operation,
                stand_name,
                e,
                log_path.display()
            );

            ExecutionResult {
                success: false,
                log_path: log_path.to_string_lossy().to_string(),
                stdout: String::new(),
            }
        }
    }
}

fn build_output_block(operation: &str, stand_name: &str, output: &Output) -> String {
    let mut block = format!(
        "[{}] Step: {} ({})\n",
        logging::message_timestamp(),
        operation,
        stand_name
    );

    if !output.stdout.is_empty() {
        block.push_str(&format!(
            "[{}] stdout:\n{}\n",
            logging::message_timestamp(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    if !output.stderr.is_empty() {
        block.push_str(&format!(
            "[{}] stderr:\n{}\n",
            logging::message_timestamp(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    block.push_str(&format!(
        "[{}] Exit code: {}",
        logging::message_timestamp(),
        output.status.code().unwrap_or(-1)
    ));

    block
}

fn write_log(log_path: &Path, content: &str, append: bool) {
    let result = if append {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .and_then(|mut file| file.write_all(content.as_bytes()))
    } else {
        fs::write(log_path, content)
    };

    if let Err(e) = result {
        log::warn!("Не удалось записать лог {}: {}", log_path.display(), e);
    }
}
