use app_lib::app::App;
use app_lib::tui::Tui;
use core::game_data::{GameData, GameWorld, UserInventory};
use std::{
    collections::HashMap,
    io::{self},
};

use ui::{home::Home, purchase_character::PurchaseCharacter, Component};

mod app_lib;
mod core;
mod ui;

fn main() -> io::Result<()> {
    let game_data = GameData::new(UserInventory::new(None), GameWorld::new(None));
    let home: Box<dyn Component> = Box::new(Home::new());
    let purchase_character: Box<dyn Component> = Box::new(PurchaseCharacter::new(Some(
        game_data.get_characters_available_for_purchase(),
    )));
    let view_table = HashMap::from([
        ("Home".to_string(), home),
        ("PurchaseCharacter".to_string(), purchase_character),
    ]);
    let tui = Tui::new()?;
    let mut app = App::new(view_table);
    app.run(tui)?;

    Ok(())
}
