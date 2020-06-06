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
    pub fn new<T>(msg: T, kind: EnzoErrorKind) -> EnzoError
    where
        T: Into<String>,
    {
        EnzoError {
            msg: msg.into(),
            kind,
        }
    }
}

impl From<io::Error> for EnzoError {
    fn from(error: io::Error) -> Self {
        EnzoError::new(format!("{}", error), EnzoErrorKind::IOError)
    }
}

impl From<serde_yaml::Error> for EnzoError {
    fn from(error: serde_yaml::Error) -> Self {
        EnzoError::new(format!("{}", error), EnzoErrorKind::ParseError)
    }
}

impl From<crossterm::ErrorKind> for EnzoError {
    fn from(error: crossterm::ErrorKind) -> Self {
        EnzoError::new(format!("{:?}", error), EnzoErrorKind::TerminalError)
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
    ParseError,
    TerminalError,
    PathDoesNotExist,
}

impl fmt::Display for EnzoErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            EnzoErrorKind::FatalError => Red.bold().paint("fatal error"),
            EnzoErrorKind::IOError => Red.bold().paint("io error"),
            EnzoErrorKind::ConfigError => Yellow.bold().paint("configuration error"),
            EnzoErrorKind::GitError => Purple.bold().paint("git error"),
            EnzoErrorKind::ParseError => Purple.bold().paint("parse error"),
            EnzoErrorKind::TerminalError => Purple.bold().paint("terminal error"),
            EnzoErrorKind::PathDoesNotExist => Purple.bold().paint("path error"),
        };

        write!(f, "{}", msg)
    }
}
