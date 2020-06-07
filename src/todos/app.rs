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

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == self.todos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn current(&self) -> &Todo {
        let i = self.state.selected().unwrap_or(0);
        &self.todos.get(i).unwrap()
    }
}
