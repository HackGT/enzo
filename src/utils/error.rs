use ansi_term::Color;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EnzoError {
    error_type: EnzoErrorType,
    msg: String,
}

impl EnzoError {
    pub fn new(msg: &str, error_type: EnzoErrorType) -> EnzoError {
        EnzoError {
            msg: msg.to_string(),
            error_type,
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
        let msg = match self.error_type {
            EnzoErrorType::FatalError => {
                format!("{}: {}", Color::Red.bold().paint("fatal error"), self.msg)
            }
            EnzoErrorType::ConfigError => format!(
                "{}: {}",
                Color::Yellow.bold().paint("configuration error"),
                self.msg
            ),
            EnzoErrorType::GitError => {
                format!("{}: {}", Color::Purple.bold().paint("git error"), self.msg)
            }
        };
        write!(f, "{}", msg)
    }
}

impl Error for EnzoError {}
