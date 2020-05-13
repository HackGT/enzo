use ansi_term::Color;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EnzoError {
    error_type: EnzoErrorType,
    msg: String,
    cause: Option<String>,
}

impl EnzoError {
    pub fn new(msg: &str, error_type: EnzoErrorType, cause: Option<String>) -> EnzoError {
        EnzoError {
            msg: msg.to_string(),
            error_type,
            cause,
        }
    }
}

#[derive(Debug)]
pub enum EnzoErrorType {
    FatalError,
    ConfigError,
    GitError,
}

impl fmt::Display for EnzoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cause_msg = match &self.cause {
            Some(val) => format!("\n{}: {}", Color::Yellow.bold().paint("cause"), val),
            None => String::new(),
        };
        let msg = match self.error_type {
            EnzoErrorType::FatalError => format!(
                "{}: {}{}",
                Color::Red.bold().paint("fatal error"),
                self.msg,
                cause_msg
            ),
            EnzoErrorType::ConfigError => format!(
                "{}: {}{}",
                Color::Yellow.bold().paint("configuration error"),
                self.msg,
                cause_msg,
            ),
            EnzoErrorType::GitError => format!(
                "{}: {}{}",
                Color::Purple.bold().paint("git error"),
                self.msg,
                cause_msg,
            ),
        };
        write!(f, "{}", msg)
    }
}

impl Error for EnzoError {}
