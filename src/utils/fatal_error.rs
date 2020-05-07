use ansi_term::Color::Red;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FatalError {
    msg: String,
}

impl FatalError {
    pub fn new(msg: &str) -> FatalError {
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

