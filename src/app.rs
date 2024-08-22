use std::{collections::HashMap, io};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    Frame,
};

use crate::component::Component;
use crate::tui::Tui;

pub enum ViewsMode {
    Home,
    PurchaseCharacter,
}

pub struct App {
    should_quit: bool,
    current_view: ViewsMode,
    components: HashMap<String, Box<dyn Component>>,
}

impl App {
    pub fn new(components: HashMap<String, Box<dyn Component>>) -> Self {
        Self {
            should_quit: false,
            current_view: ViewsMode::Home,
            components,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match self.current_view {
            ViewsMode::Home => {
                let component = self.components.get_mut(&String::from("Home")).unwrap();
                component.render(frame)
            }
            ViewsMode::PurchaseCharacter => {
                let component = self
                    .components
                    .get_mut(&String::from("PurchaseCharacter"))
                    .unwrap();
                component.render(frame)
            }
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.quit(),
                        KeyCode::Char('n') => self.current_view = ViewsMode::PurchaseCharacter,
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
