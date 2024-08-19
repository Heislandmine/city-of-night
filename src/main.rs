use std::{io, process::Command};

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
    text::{Line, Span},
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
        .margin(1)
        .split(area);
    frame.render_widget(
        Block::new()
            .border_type(BorderType::Plain)
            .borders(Borders::all()),
        area,
    );
    frame.render_widget(Paragraph::new("2124/8/5").centered(), top_bar_layout[0]);
    frame.render_widget(Paragraph::new("昼").centered(), top_bar_layout[1]);
    frame.render_widget(Paragraph::new("10日目").centered(), top_bar_layout[2]);
    frame.render_widget(Paragraph::new("残り200日").centered(), top_bar_layout[3]);
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

    let command_area_raws = Layout::new(
        Direction::Vertical,
        vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .split(layout[1]);

    let command_area_first_row = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .margin(1)
    .split(command_area_raws[0]);

    frame.render_widget(
        Paragraph::new("[100]調教").centered(),
        command_area_first_row[0],
    );

    frame.render_widget(
        Paragraph::new("[200]セーブ").centered(),
        command_area_first_row[1],
    );

    frame.render_widget(
        Paragraph::new("[300]ロード").centered(),
        command_area_first_row[2],
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
