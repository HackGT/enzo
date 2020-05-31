use std::path::PathBuf;
use crate::workspace::todo::Todo;

pub struct Project {
    name: String,
    path: PathBuf,
    src: String,
    todos: Vec<Todo>,
}

