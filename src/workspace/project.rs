use std::path::PathBuf;
use crate::workspace::todo::Todo;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    path: PathBuf,
    src: String,
    todos: Vec<Todo>,
}

