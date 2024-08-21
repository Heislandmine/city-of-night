use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::tui::Tui;

pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn render(&self, frame: &mut Frame) {
        self.render_main_ui(frame);
    }

    fn render_main_ui(&self, frame: &mut Frame) {
        let area = frame.area();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Percentage(80)])
            .split(area);

        self.render_top_bar(frame, layout[0]);

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

    fn render_top_bar(&self, frame: &mut Frame, area: Rect) {
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

    pub fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.quit(),
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self, mut tui: Tui) -> io::Result<()> {
        tui.enter()?;

        while !self.should_quit {
            tui.draw(|frame| {
                self.render(frame);
            })?;

            self.handle_events()?;
        }

        tui.exit()?;

        Ok(())
    }
}
