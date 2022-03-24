// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
pub fn log(level: LogLevel, message: &str) -> String {
    let level = format!("{:?}", level);
    format!("[{}]: {}", level.to_uppercase(), message)
}
// /// primary function for emitting logs
// pub fn log(level: LogLevel, message: &str) -> String {
//     // unimplemented!()
//     match level {
//         LogLevel::Info => info(message),
//         LogLevel::Warning => warn(message),
//         // LogLevel::Error => error(message),
//         // other => error(message),
//         _ => error(message),
//      }
// }


pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
// pub fn info(message: &str) -> String {
//     let mut msg = String::from("[INFO]: ");
//     msg.push_str(message);
//     msg
// }

pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
// pub fn warn(message: &str) -> String {
//     let mut msg = String::from("[WARNING]: ");
//     msg.push_str(message);
//     msg
// }

pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
// pub fn error(message: &str) -> String {
//     let mut msg = String::from("[ERROR]: ");
//     msg.push_str(message);
//     msg
// }
