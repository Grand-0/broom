use std::fs;
use std::path::Path;

use log::LevelFilter;
use simplelog::{
    format_description, ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode,
    WriteLogger,
};
use time::OffsetDateTime;

const LOG_TIME_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");

const FILE_TIME_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year][month][day]_[hour][minute][second]_[subsecond digits:3]");

const MESSAGE_TIME_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

const DATE_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]");

fn now_local() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
}

pub fn init(app_data_dir: &Path) -> Result<(), String> {
    let logs_dir = app_data_dir.join("logs").join("app");
    fs::create_dir_all(&logs_dir)
        .map_err(|e| format!("Не удалось создать каталог логов {}: {}", logs_dir.display(), e))?;

    let log_path = logs_dir.join(format!("app-{}.log", date_stamp()));

    let mut builder = ConfigBuilder::new();
    let _ = builder.set_time_format_custom(LOG_TIME_FORMAT);
    let _ = builder.set_time_offset_to_local();
    let config = builder.build();

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();

    #[cfg(debug_assertions)]
    loggers.push(TermLogger::new(
        LevelFilter::Warn,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ));

    let file = fs::File::create(&log_path)
        .map_err(|e| format!("Не удалось создать лог-файл {}: {}", log_path.display(), e))?;
    loggers.push(WriteLogger::new(LevelFilter::Info, config, file));

    CombinedLogger::init(loggers).map_err(|e| e.to_string())
}

pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let message = extract_panic_message(info.payload());

        let location = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown location".to_string());

        log::error!("PANIC at {}: {}", location, message);
    }));
}

fn extract_panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().map(|s| s.clone()))
        .unwrap_or_else(|| "unknown panic".to_string())
}

pub fn file_timestamp() -> String {
    now_local().format(FILE_TIME_FORMAT).unwrap_or_default()
}

pub fn message_timestamp() -> String {
    now_local()
        .format(MESSAGE_TIME_FORMAT)
        .unwrap_or_default()
}

pub fn date_stamp() -> String {
    now_local().format(DATE_FORMAT).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::OnceLock;
    use std::time::SystemTime;
    use time::PrimitiveDateTime;

    static TEST_LOG_DIR: OnceLock<TempDir> = OnceLock::new();

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let unique = format!(
                "broom_{}_{}_{}",
                label,
                std::process::id(),
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            );
            let dir = std::env::temp_dir().join(unique);
            fs::create_dir_all(&dir).unwrap();
            TempDir(dir)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn test_logger_dir() -> &'static TempDir {
        TEST_LOG_DIR.get_or_init(|| {
            let dir = TempDir::new("logging");
            init(dir.path()).expect("init должен пройти успешно");
            dir
        })
    }

    fn app_log_path(app_data_dir: &Path) -> PathBuf {
        app_data_dir
            .join("logs")
            .join("app")
            .join(format!("app-{}.log", date_stamp()))
    }

    // ── Timestamp helpers ──

    #[test]
    fn file_timestamp_has_expected_format() {
        let ts = file_timestamp();
        assert_eq!(ts.len(), 19, "file_timestamp: {}", ts);
        assert_eq!(ts.as_bytes()[8], b'_', "file_timestamp: {}", ts);
        assert_eq!(ts.as_bytes()[15], b'_', "file_timestamp: {}", ts);
        assert!(ts[..8].chars().all(|c| c.is_ascii_digit()));
        assert!(ts[9..15].chars().all(|c| c.is_ascii_digit()));
        assert!(ts[16..19].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn message_timestamp_has_expected_format() {
        let ts = message_timestamp();
        assert_eq!(ts.len(), 19, "message_timestamp: {}", ts);
        assert_eq!(ts.as_bytes()[4], b'-');
        assert_eq!(ts.as_bytes()[7], b'-');
        assert_eq!(ts.as_bytes()[10], b' ');
        assert_eq!(ts.as_bytes()[13], b':');
        assert_eq!(ts.as_bytes()[16], b':');
        assert!(ts[0..4].chars().all(|c| c.is_ascii_digit()));
        assert!(ts[5..7].chars().all(|c| c.is_ascii_digit()));
        assert!(ts[8..10].chars().all(|c| c.is_ascii_digit()));
        assert!(ts[11..13].chars().all(|c| c.is_ascii_digit()));
        assert!(ts[14..16].chars().all(|c| c.is_ascii_digit()));
        assert!(ts[17..19].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn date_stamp_has_expected_format() {
        let ds = date_stamp();
        assert_eq!(ds.len(), 10, "date_stamp: {}", ds);
        assert_eq!(ds.as_bytes()[4], b'-');
        assert_eq!(ds.as_bytes()[7], b'-');
        assert!(ds[0..4].chars().all(|c| c.is_ascii_digit()));
        assert!(ds[5..7].chars().all(|c| c.is_ascii_digit()));
        assert!(ds[8..10].chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn stamps_are_consistent() {
        let ds = date_stamp();
        let ms = message_timestamp();
        let ft = file_timestamp();

        assert!(ms.starts_with(&format!("{} ", ds)));
        assert!(ft.starts_with(&ds.replace('-', "")));
    }

    #[test]
    fn message_timestamp_is_close_to_now() {
        let ts = message_timestamp();
        let parsed = PrimitiveDateTime::parse(
            &ts,
            &time::macros::format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second]"
            ),
        )
        .expect("парсинг таймстемпа");

        let now = now_local();
        let parsed_instant = parsed.assume_offset(now.offset());
        let delta = (now - parsed_instant).whole_seconds().abs();
        assert!(
            delta <= 5,
            "таймстемп {} расходится с текущим временем на {} сек",
            ts,
            delta
        );
    }

    // ── init ──

    #[test]
    fn init_creates_app_dir_and_log_file() {
        let dir = test_logger_dir().path();
        let app_log = app_log_path(dir);

        assert!(app_log.exists(), "файл {} должен существовать", app_log.display());
        assert!(app_log.parent().unwrap().is_dir());
    }

    #[test]
    fn init_writes_records_and_filters_levels() {
        let dir = test_logger_dir().path();

        log::info!("INFO_TOKEN_42");
        log::warn!("WARN_TOKEN_42");
        log::error!("ERROR_TOKEN_42");
        log::debug!("DEBUG_TOKEN_42");
        log::trace!("TRACE_TOKEN_42");
        log::logger().flush();

        let content = fs::read_to_string(app_log_path(dir)).expect("лог-файл существует");
        assert!(content.contains("INFO_TOKEN_42"), "содержимое: {}", content);
        assert!(content.contains("WARN_TOKEN_42"), "содержимое: {}", content);
        assert!(content.contains("ERROR_TOKEN_42"), "содержимое: {}", content);
        assert!(!content.contains("DEBUG_TOKEN_42"), "содержимое: {}", content);
        assert!(!content.contains("TRACE_TOKEN_42"), "содержимое: {}", content);
    }

    #[test]
    fn init_fails_on_second_call() {
        test_logger_dir();
        let fresh = TempDir::new("logging_twice");
        let second = init(fresh.path());
        assert!(second.is_err(), "повторный init должен вернуть Err");
    }

    #[test]
    fn init_fails_when_logs_dir_cannot_be_created() {
        let dir = TempDir::new("logging_baddir");
        fs::write(dir.path().join("logs"), "file instead of dir").unwrap();

        let result = init(dir.path());
        assert!(result.is_err());
    }

    // ── Panic hook ──

    #[test]
    fn extracts_message_from_str_payload() {
        assert_eq!(extract_panic_message(&"boom"), "boom");
    }

    #[test]
    fn extracts_message_from_string_payload() {
        assert_eq!(extract_panic_message(&String::from("boom")), "boom");
    }

    #[test]
    fn extracts_unknown_message_from_other_payload() {
        assert_eq!(extract_panic_message(&42i32), "unknown panic");
    }

    #[test]
    fn panic_hook_writes_to_log() {
        let dir = test_logger_dir().path();
        let previous = std::panic::take_hook();
        install_panic_hook();

        let handle = std::thread::spawn(|| panic!("PANIC_TOKEN_42"));
        assert!(handle.join().is_err());

        std::panic::set_hook(previous);
        log::logger().flush();

        let content = fs::read_to_string(app_log_path(dir)).expect("лог-файл существует");
        assert!(content.contains("PANIC at"), "содержимое: {}", content);
        assert!(content.contains("PANIC_TOKEN_42"), "содержимое: {}", content);
    }
}
