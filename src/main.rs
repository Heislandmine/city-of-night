use app::App;
use std::io::{self};
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

        app.handle_events()?;
    }

    tui.exit()?;

    Ok(())
}
