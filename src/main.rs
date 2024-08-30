use app_lib::app::App;
use app_lib::tui::Tui;
use core::{
    game_controller::GameController,
    game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory},
};
use std::io::{self};
use ui::user_view::UserView;

mod app_lib;
mod core;
mod ui;

fn main() -> io::Result<()> {
    let view = UserView::new();
    let tui = Tui::new()?;
    let controller = GameController::new(
        UserInventory::new(None),
        GameWorld::new(None),
        Vec::new(),
        vec![CharactersAvailableForPurchase::new(
            "demo-ko".to_string(),
            "1".to_string(),
            "デモ子".to_string(),
            2000,
        )],
    );
    let mut app = App::new(
        UserInventory::new(None),
        GameWorld::new(None),
        controller,
        view,
    );
    app.run(tui)?;

    Ok(())
}
