use ansi_term::Color::{Purple, Red, Yellow};
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct EnzoError {
    kind: EnzoErrorKind,
    msg: String,
}

impl EnzoError {
    pub fn new(msg: String, kind: EnzoErrorKind) -> EnzoError {
        EnzoError { msg, kind }
    }
}

impl From<io::Error> for EnzoError {
    fn from(error: io::Error) -> Self {
        EnzoError::new(format!("{}", error), EnzoErrorKind::IOError)
    }
}

impl fmt::Display for EnzoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl Error for EnzoError {}

#[derive(Debug)]
pub enum EnzoErrorKind {
    FatalError,
    IOError,
    ConfigError,
    GitError,
}

impl fmt::Display for EnzoErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            EnzoErrorKind::FatalError => Red.bold().paint("fatal error"),
            EnzoErrorKind::IOError => Red.bold().paint("io error"),
            EnzoErrorKind::ConfigError => Yellow.bold().paint("configuration error"),
            EnzoErrorKind::GitError => Purple.bold().paint("git error"),
        };

        write!(f, "{}", msg)
    }
}

