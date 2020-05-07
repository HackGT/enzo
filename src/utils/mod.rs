pub mod fatal_error;

use ansi_term::Color;
use dirs::home_dir;
use fatal_error::FatalError;
use std::path::PathBuf;

pub fn get_home_dir() -> Result<PathBuf, FatalError> {
    match home_dir() {
        Some(path) => Ok(path),
        None => Err(FatalError::new("Couldn't access the home directory")),
    }
}

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
