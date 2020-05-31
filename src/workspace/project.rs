use crate::workspace::todo::Todo;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    path: PathBuf,
    src: String,
    todos: Vec<Todo>,
}

impl Project {
    pub fn new(path: PathBuf, src: String, todos: Vec<Todo>) -> Self {
        Self {
            name: String::from("dummy name"),
            path,
            src,
            todos,
        }
    }
}
