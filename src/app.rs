use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    Frame,
};

use crate::home::Home;
use crate::tui::Tui;

pub enum ViewsMode {
    HOME,
}

pub struct App {
    pub should_quit: bool,
    pub current_view: ViewsMode,
    component: Home,
}

impl App {
    pub fn new(component: Home) -> Self {
        Self {
            should_quit: false,
            current_view: ViewsMode::HOME,
            component,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn render(&self, frame: &mut Frame) {
        match self.current_view {
            ViewsMode::HOME => self.component.render(frame),
        }
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
