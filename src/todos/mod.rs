pub mod app;
pub mod todo;
mod ui;

use crate::{
    config::project::ProjectConfig,
    utils::{
        error::{EnzoError, EnzoErrorKind},
        query::{AnswerKind, Question},
    },
};
use app::App;
use crossterm::{
    event::{EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use futures::{executor::block_on, StreamExt};
use std::{
    fs::File,
    io::{prelude::*, stdout, Write},
    path::PathBuf,
};
use todo::Todo;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn start<'a>(todos: &'a mut Vec<Todo>) -> Result<(), EnzoError> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let mut app = App::with_todos(todos);
    app.state.select(Some(0));

    block_on(event_listener(&mut terminal, &mut app))?;

    disable_raw_mode()?;
    Ok(())
}

pub fn read_from(path: &PathBuf) -> Result<Vec<Todo>, EnzoError> {
    if !path.exists() {
        return Err(EnzoError::new(
            format!("The path {:?} does not exist", path),
            EnzoErrorKind::PathDoesNotExist,
        ));
    }
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let ProjectConfig { todos, .. } = serde_yaml::from_str(buffer.as_str())?;
    // TODO remove the unwrap
    Ok(todos.unwrap())
}

async fn event_listener<T: Backend>(
    terminal: &mut Terminal<T>,
    app: &mut App<'_>,
) -> Result<(), EnzoError> {
    let mut reader = EventStream::new();

    terminal.draw(|mut f| ui::draw(&mut f, app))?;
    while let Some(event) = reader.next().await {
        match event {
            Ok(event) => match event {
                Event::Key(k) => {
                    if k == KeyCode::Char('q').into() {
                        break;
                    } else if k == KeyCode::Char('j').into() || k == KeyCode::Down.into() {
                        app.next();
                    } else if k == KeyCode::Char('k').into() || k == KeyCode::Up.into() {
                        app.previous();
                    } else if k == KeyCode::Enter.into() {
                        if app.current().is_complete() {
                            app.current_mut().mark_incomplete();
                        } else {
                            app.current_mut().mark_complete();
                        }
                    } else {
                        // TODO
                    }
                }
                _ => break,
            },
            Err(_) => eprintln!("error"),
        }
        terminal.draw(|mut f| ui::draw(&mut f, app))?;
    }
    Ok(())
}
