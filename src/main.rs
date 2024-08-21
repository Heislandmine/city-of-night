use app::App;
use std::io::{self};
use tui::Tui;

mod app;
mod tui;

fn main() -> io::Result<()> {
    let tui = Tui::new()?;
    let mut app = App::new();

    app.run(tui)?;

    Ok(())
}
