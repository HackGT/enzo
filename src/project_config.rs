use crate::workspace::todo::Todo;
use crate::utils::error::EnzoError;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    todos: Vec<Todo>,
}

impl ProjectConfig {
    fn read(&mut self, path: PathBuf) -> Result<(), EnzoError> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let ProjectConfig { ref todos, ..} = serde_yaml::from_str(buffer.as_str())?; 
        self.todos = todos.clone();
        Ok(())
    }

    fn write(&mut self, path: PathBuf) -> Result<(), EnzoError> {
        let mut file = File::open(path)?;
        let s = serde_yaml::to_string(&self)?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }
}
