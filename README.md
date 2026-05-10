# log-overflow
A simple and lightweight logging library written in rust.
 
 ## How to use?
 First, make sure to import the crate, initialize the logger and you can start logging!
 
 Example:
 
 ```rust
 use log_overflow{log_init, log, Severity};
 
 fn main() {
     log_init("foobartool_v1", "/tmp/foobartool/", true);
     log(Severity::TRACE, "hello");
     log(Severity::DEBUG, "debug");
     log(Severity::INFO, "info");
     log(Severity::WARNING, "warning");
     log(Severity::CRITICAL, "critical");
 }
 ```

## Why use over other libraries?

log-overflow is written in rust, so it is native to this programming language, it is blazing fast, extremely simple and barely uses any resources while still having the features of a big logging library.