use std::io::{self, Stdout};

use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    CompletedFrame, Frame, Terminal,
};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout()))?,
        })
    }

    pub fn enter(&self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;

        Ok(())
    }

    pub fn exit(&self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;

        Ok(())
    }

    pub fn draw<F>(&mut self, render_callback: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        Ok(self.terminal.draw(render_callback)?)
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
