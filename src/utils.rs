use ansi_term::Color::Red;
use dirs::home_dir;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FatalError {
    msg: String,
}

impl FatalError {
    fn new(msg: &str) -> FatalError {
        FatalError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", Red.bold().paint("fatal error"), self.msg)
    }
}

impl Error for FatalError {}

pub fn get_home_dir() -> Result<PathBuf, FatalError> {
    match home_dir() {
        Some(path) => Ok(path),
        None => Err(FatalError::new("Couldn't access the home directory")),
    }
}
