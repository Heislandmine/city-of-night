use app::App;
use home::Home;
use std::io::{self};
use tui::Tui;

mod app;
mod home;
mod tui;

fn main() -> io::Result<()> {
    let tui = Tui::new()?;
    let home = Home::new();
    let mut app = App::new(home);

    app.run(tui)?;

    Ok(())
}
