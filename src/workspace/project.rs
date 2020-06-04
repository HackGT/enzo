use crate::todos::todo::Todo;
use crate::workspace::WorkspaceName;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    src: String,
    workspace: WorkspaceName,
    pub todos: Vec<Todo>,
}

impl Project {
    pub fn new(name: String, workspace: WorkspaceName, src: String, todos: Vec<Todo>) -> Self {
        Self {
            name,
            workspace,
            src,
            todos,
        }
    }
}
