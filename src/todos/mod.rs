pub mod app;
pub mod todo;
mod ui;

use crate::config::project::ProjectConfig;
use crate::utils::error::EnzoError;
use app::{App, StatefulList};
use crossterm::{
    event::{EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use futures::{executor::block_on, StreamExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, Write};
use std::path::PathBuf;
use todo::Todo;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn start() -> Result<(), EnzoError> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App {
        items: StatefulList::with_items(vec![
            Todo::new("study".to_string()),
            Todo::new("have a break".to_string()),
            Todo::new("perhaps not have a break".to_string()),
        ]),
    };

    block_on(event_listener(&mut terminal, &mut app));

    disable_raw_mode()?;
    Ok(())
}

async fn event_listener<T: Backend>(terminal: &mut Terminal<T>, app: &mut App) {
    let mut reader = EventStream::new();

    terminal.draw(|mut f| ui::draw(&mut f, app));
    while let Some(event) = reader.next().await {
        match event {
            Ok(event) => {
                if event == Event::Key(KeyCode::Char('q').into()) {
                    break;
                } else if event == Event::Key(KeyCode::Down.into()) {
                    app.items.next();
                } else {
                    println!("this is an event\r");
                }
            }
            Err(_) => eprintln!("error"),
        }
        terminal.draw(|mut f| ui::draw(&mut f, app));
    }
}

pub fn read_from(path: &PathBuf) -> Result<Vec<Todo>, EnzoError> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let ProjectConfig { todos, .. } = serde_yaml::from_str(buffer.as_str())?;
    Ok(todos)
}
