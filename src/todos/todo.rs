use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
}

impl Todo {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            completed: Some(false),
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = Some(true);
    }

    pub fn mark_incomplete(&mut self) {
        self.completed = Some(false);
    }
}

