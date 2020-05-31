pub struct Todo {
    name: String,
    completed: bool,
}

impl Todo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            completed: false,
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }

    pub fn mark_incomplete(&mut self) {
        self.completed = false;
    }
}
