use super::section::{self, Instruction, Section};
use crate::{
    todos::todo::Todo,
    utils::error::{EnzoError, EnzoErrorKind},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::prelude::*, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub todos: Option<Vec<Todo>>,
    pub configure: Option<HashMap<Section, Vec<Instruction>>>,
}

impl ProjectConfig {
    pub fn configure(&self) -> Result<(), EnzoError> {
        if let Some(ref mapping) = self.configure {
            for (section, instructions) in mapping.iter() {
                println!("Executing section {:?}", section);
                section::execute(instructions)?;
            }
        }
        Ok(())
    }
}

pub fn read_from(path: &PathBuf) -> Result<ProjectConfig, EnzoError> {
    if !path.exists() {
        return Err(EnzoError::new(
            format!("The path {:?} does not exist", path),
            EnzoErrorKind::PathDoesNotExist,
        ));
    }

    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let project_config = serde_yaml::from_str(&buffer)?;
    Ok(project_config)
}
