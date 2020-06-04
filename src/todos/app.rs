use crate::todos::Todo;
use tui::widgets::ListState;

pub struct App<'a> {
    pub todos: &'a Vec<Todo>,
    pub state: ListState,
}

impl<'a> App<'a> {
    pub fn with_todos(todos: &'a mut Vec<Todo>) -> Self {
        App {
            todos,
            state: ListState::default(),
        }
    }
}
