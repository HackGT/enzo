use crate::todos::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Text},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    let items = app.todos.iter().map(|i| Text::raw(i.name.clone()));

    let style = Style::default().fg(Color::Black).bg(Color::White);

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .style(style)
        .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
        .highlight_symbol(">");
    f.render_stateful_widget(items, chunks[0], &mut app.state);
}
