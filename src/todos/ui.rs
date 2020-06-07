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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("todos")
                .title_style(Style::new().fg(Color::Yellow).modifier(Modifier::BOLD)),
        )
        .style(style)
        .highlight_style(style.fg(Color::Blue).modifier(Modifier::BOLD))
        .highlight_symbol(">>= ");

    // draw description
    let status = if app.current().is_complete() {
        Text::styled(
            "\ncomplete\n\n",
            Style::new().fg(Color::Green).modifier(Modifier::BOLD),
        )
    } else {
        Text::styled(
            "\nincomplete\n\n",
            Style::new().fg(Color::Red).modifier(Modifier::BOLD),
        )
    };
    let mut description = String::new();
    if let Some(s) = app.current().description.as_ref() {
        description.push_str(&s.clone())
    };
    let text = [status, Text::raw(format!("{}", description))];
    let current_description = Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("description")
                .title_style(Style::new().fg(Color::Magenta).modifier(Modifier::BOLD)),
        )
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("help")
                .title_style(Style::new().fg(Color::White).modifier(Modifier::BOLD)),
        )
        .style(style);

    f.render_stateful_widget(items, chunks[0], &mut app.state);
    f.render_widget(current_description, chunks[1]);
    f.render_widget(help, chunks[2]);
}
