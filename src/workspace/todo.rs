use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    name: String,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(name: String) -> Self {
        Self {
            name,
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
