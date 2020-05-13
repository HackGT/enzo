pub mod error;

use ansi_term::Color;
use clap::ArgMatches;
use dirs::home_dir;
use error::{EnzoError, EnzoErrorType};
use std::path::PathBuf;

pub fn get_home_dir() -> Result<PathBuf, EnzoError> {
    match home_dir() {
        Some(path) => Ok(path),
        None => Err(EnzoError::new(
            "Couldn't access the home directory",
            EnzoErrorType::FatalError,
            None,
        )),
    }
}

// TODO add support for hints
pub fn query(question: &str, default: Option<&str>, pre: Option<&str>) -> String {
    format!(
        "{} {} {}\n> {}",
        Color::Green.bold().paint("?"),
        Color::White.bold().paint(question),
        if let Some(default) = default {
            format!("(default {})", Color::White.dimmed().paint(default))
        } else {
            String::new()
        },
        if let Some(pre) = pre {
            format!("{}", Color::Yellow.paint(pre))
        } else {
            String::new()
        }
    )
}

pub fn get<'a>(key: &'a str, input: &'a ArgMatches) -> Result<&'a str, EnzoError> {
    match input.value_of(key) {
        Some(val) => Ok(val),
        None => Err(EnzoError::new(
            format!("Could not obtain {} from arg matches", key).as_str(),
            EnzoErrorType::FatalError,
            None,
        )),
    }
}
