use std::io::{self};

use app::App;
use ratatui::crossterm::event::{self, KeyCode, KeyEventKind};
use tui::Tui;

mod app;
mod tui;

fn main() -> io::Result<()> {
    let mut tui = Tui::new()?;
    let mut app = App::new();
    tui.enter()?;

    while !app.should_quit {
        tui.draw(|frame| {
            app.render(frame);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    app.quit();
                }
            }
        }
    }

    // restore terminal
    tui.exit()?;

    Ok(())
}
