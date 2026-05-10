// SPDX-License-Identifier: MIT OR Apache-2.0
// lib.rs - log-overflow - angrypig555
//! # log-overflow - A simple and lightweight logging library.
//! 
//! ## How to use?
//! First, make sure to import the crate, initialize the logger and you can start logging!
//! 
//! Example:
//! 
//! ```rust
//! use log_overflow{log_init, log, Severity};
//! 
//! fn main() {
//!     log_init("foobartool_v1", "/tmp/foobartool/", true);
//!     log(Severity::TRACE, "hello");
//!     log(Severity::DEBUG, "debug");
//!     log(Severity::INFO, "info");
//!     log(Severity::WARNING, "warning");
//!     log(Severity::CRITICAL, "critical");
//! }
//! ```
//! 
//! Why use over other libraries?
//! 
//! log-overflow is written in rust, so it is native to this programming language, it is blazing fast, extremely simple and barely uses any resources while still having the features of a big logging library.
use std::path::{PathBuf};
use std::fs::{create_dir_all, OpenOptions};
use std::fs;
use std::io::{Write};
use chrono::{Local};
use std::sync::OnceLock;
use std::sync::atomic::{Ordering, AtomicBool};
use std::thread;
use std::process;

static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();
static LOGGING_ENABLED: AtomicBool = AtomicBool::new(false);
static SOFTWARE_NAME: OnceLock<String> = OnceLock::new();

/// Severity of a log
/// 
/// # Severity
/// 
/// The severity can either be:
/// - TRACE
/// - DEBUG
/// - INFO
/// - WARNING
/// - CRITICAL
/// 
/// They can be used with the `log()` function and is useful for reading the logs.
#[derive(Debug, PartialEq, Eq)]
pub enum Severity {
    TRACE,
    DEBUG,
    INFO,
    WARNING,
    CRITICAL,
}

/// Set a configuration value
/// 
/// # Set a configuration value
/// 
/// Arguments:
/// - name - Name of the configuration value
/// - value - Value to set configuration
/// 
/// Usage of this function is not reccomended by its own, please use `log_init()` to set config values at start.
/// The `log_init()` function handles the configuration.
/// 
/// ## Panics
/// This function will panic if:
/// - It fails to get the PathBuf from the value when setting LOG_PATH
/// - When LOGGING_ENABLED is not a boolean
/// - When it fails to set the software name
/// - When the configuration name does not exist
pub fn conf_set(name: &str, value: &str) {
    if name == "LOG_PATH" {
        LOG_PATH.set(PathBuf::from(value)).expect("(log-overflow) Could not set LOG_PATH");
    } else if name == "LOGGING_ENABLED" {
        LOGGING_ENABLED.store(value.parse().expect("(log-overflow) value is not a boolean"), Ordering::Relaxed);
    } else if name == "SOFTWARE_NAME" {
        SOFTWARE_NAME.set(value.to_owned()).expect("(log-overflow) Failed to set software name");
    } else {
        panic!("(log-overflow) config name does not exist {}", name);
    }
}

/// Initialize log-overflow
/// 
/// # Initialize log-overflow
/// 
/// Arguments:
/// - software_name - The name of your software, can be any string.
/// - log_path - Which folder to put the log in.
/// - logging_enable - Enable / disable logging. (boolean)
/// 
/// Does not return anything, only returns if logging is not enabled
/// 
/// Creates a log file structured like:
/// ```
/// my_program_name - process_id - log-overflow - time
/// time - thread_id - severity - message
/// ```
/// 
/// Do not run this function more than once as it can break the OnceLock.
/// 
/// ## Panics
/// This function will panic if:
/// - It fails to create the logging directory
/// - Fails to write log header
/// - Fails to remove
/// - Ran more than once
pub fn log_init(software_name: &str, log_path: &str, logging_enable: bool) {
    let log_folder = shellexpand::tilde(log_path).into_owned();
    let path = PathBuf::from(log_folder);
    let curr_time = Local::now();
    let curr_time_web_safe = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let pid = process::id();
    let log_name = format!("{}-{}", curr_time_web_safe, pid);
    let log_file = path.join(format!("{}.log", log_name));
    conf_set("LOG_PATH", log_file.to_str().unwrap());
    conf_set("SOFTWARE_NAME", software_name);
    LOGGING_ENABLED.store(logging_enable, Ordering::Relaxed);
    if !logging_enable {
        return;
    }
    if let Some(software_name) = SOFTWARE_NAME.get() {
        let log_header = format!("{} - {} - log-overflow - {}\n", software_name, pid, curr_time);
        if !path.exists() {
            create_dir_all(path).expect("(log-overflow) Failed to create logging directory");
        }
        if !log_file.exists() {
            fs::write(&log_file, &log_header).expect("(log-overflow) Failed to write header to log file");
        }
    } else {
        panic!("(log-overflow) software_name is not set!");
    }
    
    
}


/// Log something
/// 
/// # Log something
/// 
/// Arguments:
/// - severity - Severity of the log (see Severity for the levels of severity)
/// - message - The log message, if you want to format the string make sure to use format!()
/// 
/// Returns nothing, will not run if logging is disabled in log_init.
/// 
/// 
/// ## Panics
/// This function will panic if:
/// - It fails to write to the log file.
pub fn log(severity: Severity, msg: &str) {
    if !LOGGING_ENABLED.load(Ordering::Relaxed) {
        return;
    }
    let severity_string = match severity {
        Severity::TRACE => "TRACE",
        Severity::DEBUG => "DEBUG",
        Severity::INFO => "INFO",
        Severity::WARNING => "WARNING",
        Severity::CRITICAL => "CRITICAL",
    };
    let curr_time = Local::now();
    let message = format!("{} - {:?}: [{}] {}\n", curr_time, thread::current().id(), severity_string, msg);
    if let Some(path) = LOG_PATH.get() {
        let file = OpenOptions::new().create(true).append(true).open(path);
        if let Ok(mut file) = file {
            file.write_all(message.as_bytes()).expect("(log-overflow) Failed to write to log file");
        }
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logtest() {
        log_init("foobartool_v1", "/tmp/foobartool/", true);
        log(Severity::TRACE, "hello");
        log(Severity::DEBUG, "debug");
        log(Severity::INFO, "info");
        log(Severity::WARNING, "warning");
        log(Severity::CRITICAL, "critical");
    }
}
