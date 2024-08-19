use std::io;

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    symbols::{block, border},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

fn render_top_bar(frame: &mut Frame, area: Rect) {
    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    frame.render_widget(
        Paragraph::new("2124/8/5").centered().block(
            Block::new()
                .border_type(BorderType::Plain)
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM),
        ),
        top_bar_layout[0],
    );
    frame.render_widget(
        Paragraph::new("昼").centered().block(
            Block::new()
                .border_type(BorderType::Plain)
                .borders(Borders::TOP | Borders::BOTTOM),
        ),
        top_bar_layout[1],
    );
    frame.render_widget(
        Paragraph::new("10日目").centered().block(
            Block::new()
                .border_type(BorderType::Plain)
                .borders(Borders::TOP | Borders::BOTTOM),
        ),
        top_bar_layout[2],
    );
    frame.render_widget(
        Paragraph::new("残り200日").centered().block(
            Block::new()
                .border_type(BorderType::Plain)
                .borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT),
        ),
        top_bar_layout[3],
    );
}

fn render_main_ui(frame: &mut Frame) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);

    render_top_bar(frame, layout[0]);

    frame.render_widget(
        Block::new()
            .border_type(BorderType::Plain)
            .borders(Borders::all())
            .border_style(Style::default().fg(Color::White)),
        layout[1],
    );
}
fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| {
            render_main_ui(frame);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
