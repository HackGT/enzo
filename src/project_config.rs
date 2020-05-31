use crate::utils::error::EnzoError;
use crate::workspace::todo::Todo;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    todos: Vec<Todo>,
}

impl ProjectConfig {
    fn read(&mut self, path: PathBuf) -> Result<(), EnzoError> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let ProjectConfig { ref todos, .. } = serde_yaml::from_str(buffer.as_str())?;
        self.todos = todos.clone();
        Ok(())
    }
}
