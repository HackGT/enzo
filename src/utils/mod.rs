pub mod error;
pub mod query;

use clap::ArgMatches;
use dirs::home_dir;
use error::{EnzoError, EnzoErrorKind};
use std::path::PathBuf;

pub fn get_home_dir() -> Result<PathBuf, EnzoError> {
    match home_dir() {
        Some(path) => Ok(path),
        None => Err(EnzoError::new(
            "Couldn't access the home directory".to_string(),
            EnzoErrorKind::FatalError,
        )),
    }
}

pub fn get<'a>(key: &'a str, input: &'a ArgMatches) -> Result<&'a str, EnzoError> {
    match input.value_of(key) {
        Some(val) => Ok(val),
        None => Err(EnzoError::new(
            format!("Could not obtain {} from arg matches", key),
            EnzoErrorKind::FatalError,
        )),
    }
}

pub fn warning(msg: &str) {
    println!(
        "{} {}",
        ansi_term::Color::Yellow.bold().paint("[warning]"),
        msg
    );
}

pub fn info(msg: &str) {
    println!("{} {}", ansi_term::Color::Blue.bold().paint("[info]"), msg)
}

pub fn success(msg: &str) {
    println!(
        "{} {}",
        ansi_term::Color::Green.bold().paint("[success]"),
        msg
    )
}
