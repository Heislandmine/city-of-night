use std::{collections::HashMap, io};

use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        execute,
        style::Print,
    },
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
    string_inputted: String,
    components: HashMap<String, Box<dyn Component>>,
}

impl App {
    pub fn new(components: HashMap<String, Box<dyn Component>>) -> Self {
        Self {
            should_quit: false,
            current_view: ViewsMode::Home,
            string_inputted: String::new(),
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
                component.render(frame, &self.string_inputted)
            }
            ViewsMode::PurchaseCharacter => {
                let component = self
                    .components
                    .get_mut(&String::from("PurchaseCharacter"))
                    .unwrap();
                component.render(frame, &self.string_inputted)
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
                        KeyCode::Char(c) => self.string_inputted.push(c),
                        KeyCode::Backspace => {
                            let _ = self.string_inputted.pop();
                        }
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
