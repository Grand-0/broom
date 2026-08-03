use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

const RETENTION_DAYS: u64 = 7;
const DAY_SECONDS: u64 = 24 * 60 * 60;
const LOG_SUBDIRS: [&str; 2] = ["operations", "app"];

fn log_subdir_path(app_data_dir: &Path, subdir: &str) -> PathBuf {
    app_data_dir.join("logs").join(subdir)
}

pub fn cleanup_stale_logs(app_data_dir: &Path) {
    for subdir in LOG_SUBDIRS {
        let dir = log_subdir_path(app_data_dir, subdir);
        if !dir.exists() {
            continue;
        }

        let cutoff = SystemTime::now() - Duration::from_secs(RETENTION_DAYS * DAY_SECONDS);

        match fs::read_dir(&dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    remove_if_stale(&entry.path(), cutoff);
                }
            }
            Err(e) => {
                log::warn!("Не удалось прочитать каталог логов {}: {}", dir.display(), e);
            }
        }
    }
}

fn remove_if_stale(path: &Path, cutoff: SystemTime) {
    let Ok(metadata) = fs::metadata(path) else { return };
    if !metadata.is_file() {
        return;
    }

    let Ok(modified) = metadata.modified() else { return };
    if modified < cutoff {
        match fs::remove_file(path) {
            Ok(()) => log::info!("Удалён устаревший лог: {}", path.display()),
            Err(e) => log::warn!("Не удалось удалить лог {}: {}", path.display(), e),
        }
    }
}

pub fn start_daily_cleanup(app_data_dir: PathBuf) {
    std::thread::spawn(move || loop {
        cleanup_stale_logs(&app_data_dir);
        std::thread::sleep(Duration::from_secs(DAY_SECONDS));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn log_subdir_paths_are_computed_correctly() {
        let dir = TempDir::new("log_cleanup_paths");
        let root = dir.path();

        assert_eq!(
            log_subdir_path(root, "operations"),
            root.join("logs").join("operations")
        );
        assert_eq!(log_subdir_path(root, "app"), root.join("logs").join("app"));
    }

    #[test]
    fn cleanup_keeps_fresh_files() {
        let dir = TempDir::new("log_cleanup_fresh");
        let root = dir.path();

        for subdir in LOG_SUBDIRS {
            let sub = root.join("logs").join(subdir);
            fs::create_dir_all(&sub).unwrap();
            fs::write(sub.join(format!("{}.txt", subdir)), "recent content").unwrap();
        }

        cleanup_stale_logs(root);

        for subdir in LOG_SUBDIRS {
            let file = root
                .join("logs")
                .join(subdir)
                .join(format!("{}.txt", subdir));
            assert!(
                file.exists(),
                "свежий файл {} не должен удаляться",
                file.display()
            );
        }
    }

    #[test]
    fn cleanup_removes_stale_files_and_keeps_fresh() {
        let dir = TempDir::new("log_cleanup_stale");
        let root = dir.path();

        let operations = root.join("logs").join("operations");
        fs::create_dir_all(&operations).unwrap();

        let stale = operations.join("stale.txt");
        let fresh = operations.join("fresh.txt");

        fs::write(&stale, "old content").unwrap();
        let stale_age = SystemTime::now() - Duration::from_secs(8 * DAY_SECONDS);
        fs::OpenOptions::new()
            .write(true)
            .open(&stale)
            .unwrap()
            .set_modified(stale_age)
            .unwrap();

        fs::write(&fresh, "recent content").unwrap();

        cleanup_stale_logs(root);

        assert!(
            !stale.exists(),
            "устаревший лог {} должен удаляться",
            stale.display()
        );
        assert!(
            fresh.exists(),
            "свежий лог {} не должен удаляться",
            fresh.display()
        );
    }
}
