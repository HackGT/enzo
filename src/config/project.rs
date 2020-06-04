use crate::todos::todo::Todo;
use crate::utils::error::EnzoError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

// TODO: Remove this nasty read function, and make this not in charge of file io
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub todos: Vec<Todo>,
}

impl ProjectConfig {
    pub fn read(&mut self, path: &PathBuf) -> Result<(), EnzoError> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let ProjectConfig { ref todos, .. } = serde_yaml::from_str(buffer.as_str())?;
        self.todos = todos.clone();
        Ok(())
    }
}
