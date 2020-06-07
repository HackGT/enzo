use crate::todos::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Paragraph, Text},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(50),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(f.size());

    // draw todos
    let items = app.todos.iter().map(|i| {
        if i.completed.unwrap_or_default() {
            Text::styled(
                i.name.clone(),
                Style::new().modifier(Modifier::DIM | Modifier::CROSSED_OUT),
            )
        } else {
            Text::raw(i.name.clone())
        }
    });
    let style = Style::default();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("todos"))
        .style(style)
        .highlight_style(style.fg(Color::Blue).modifier(Modifier::BOLD))
        .highlight_symbol(">>= ");

    // draw description
    let description = match app.current().description.as_ref() {
        Some(s) => s.clone(),
        None => String::new(),
    };
    let text = [Text::raw(format!("{}", description))];
    let current_description = Paragraph::new(text.iter())
        .block(Block::default().borders(Borders::ALL).title("description"))
        .style(style);

    // draw help
    let help_text = [
        Text::raw("down arrow / j - next todo\n"),
        Text::raw("up arrow / k - previous todo\n"),
        Text::raw("enter - check/uncheck todo\n"),
        Text::raw("a - add todo\n"),
        Text::raw("d - remove todo\n"),
    ];
    let help = Paragraph::new(help_text.iter())
        .block(Block::default().borders(Borders::ALL).title("help"))
        .style(style);

    f.render_stateful_widget(items, chunks[0], &mut app.state);
    f.render_widget(current_description, chunks[1]);
    f.render_widget(help, chunks[2]);
}
