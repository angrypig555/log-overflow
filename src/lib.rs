use std::path::{PathBuf};
use std::fs::{create_dir_all, OpenOptions};
use std::fs;
use std::io::{Write};
use chrono::{Local};
use std::sync::OnceLock;
use std::sync::atomic::Ordering;
use std::thread;

static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();
static LOGGING_ENABLED: OnceLock<bool> = OnceLock::new();

/// Initialize log-overflow
/// 
/// Arguments:
/// - raw_path - Path to the log folder, can contain tildes as it will be resolved by shellexpand
/// 
/// Does not return anything.
/// 
/// Creates a log file structured like:
/// ```
/// my_program_name - version - logs - time
/// time - thread_id - severity - message
/// ```
pub fn log_init(raw_path: &str) {
    let log_folder = shellexpand::tilde(raw_path).into_owned();
    let path = PathBuf::from(log_folder);

    let log_file = path.join("rusttp.log");
    let curr_time = Local::now();
    let log_header = format!("rusttp v1.0 - logs - {}\n", curr_time);
    if !path.exists() {
        create_dir_all(path);
    }
    if !log_file.exists() {
        fs::write(&log_file, &log_header);
    }
    if log_file.exists() {
        fs::remove_file(&log_file);
        fs::write(&log_file, &log_header);
    }
    LOG_PATH.set(log_file);
}

pub fn log(msg: &str) {
    if !LOGGING_ENABLED.load(Ordering::Relaxed) {
        return;
    }
    let curr_time = Local::now();
    let message = format!("{:?} - {}: {}\n", thread::current().id(), curr_time, msg);
    if let Some(path) = LOG_PATH.get() {
        let file = OpenOptions::new().create(true).append(true).open(path);
        if let Ok(mut file) = file {
            let _ = file.write_all(message.as_bytes());
        }
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logtest() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
