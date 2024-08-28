use app_lib::app::App;
use app_lib::tui::Tui;
use core::game_data::{GameWorld, UserInventory};
use std::io::{self};
use ui::user_view::UserView;

mod app_lib;
mod core;
mod ui;

fn main() -> io::Result<()> {
    let view = UserView::new();
    let tui = Tui::new()?;
    let mut app = App::new(UserInventory::new(None), GameWorld::new(None), view);
    app.run(tui)?;

    Ok(())
}
