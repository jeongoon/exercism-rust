#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// Primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let levelstr = match level {
        LogLevel::Info    => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error   => "ERROR",
    };

    "[".to_owned() + levelstr + "]: " + message
}

pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

fn main() {
    println!( "{}", log(LogLevel::Info, "This is info") );
    println!( "{}", log(LogLevel::Warning, "Not enough time!!!!") );
}
