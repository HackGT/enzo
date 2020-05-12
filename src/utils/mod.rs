pub mod error;

use ansi_term::Color;
use dirs::home_dir;
use error::{EnzoError, EnzoErrorType};
use std::path::PathBuf;

pub fn get_home_dir() -> Result<PathBuf, EnzoError> {
    match home_dir() {
        Some(path) => Ok(path),
        None => Err(EnzoError::new(
            "Couldn't access the home directory",
            EnzoErrorType::FatalError,
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
