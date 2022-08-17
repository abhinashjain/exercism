// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    //unimplemented!()
    return match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        _ => error(message),
    }
}
pub fn info(message: &str) -> String {
    //unimplemented!()
    return "[INFO]: ".to_owned() + message
}
pub fn warn(message: &str) -> String {
    //unimplemented!()
    return "[WARNING]: ".to_owned() + message
}
pub fn error(message: &str) -> String {
    //unimplemented!()
    return "[ERROR]: ".to_owned() + message
}

