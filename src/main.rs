use app_lib::app::App;
use app_lib::tui::Tui;
use std::{
    collections::HashMap,
    io::{self},
};

use ui::{home::Home, purchase_character::PurchaseCharacter, Component};

mod app_lib;
mod core;
mod ui;

fn main() -> io::Result<()> {
    let home: Box<dyn Component> = Box::new(Home::new());
    let purchase_character: Box<dyn Component> = Box::new(PurchaseCharacter::new());
    let view_table = HashMap::from([
        ("Home".to_string(), home),
        ("PurchaseCharacter".to_string(), purchase_character),
    ]);
    let tui = Tui::new()?;
    let mut app = App::new(view_table);
    app.run(tui)?;

    Ok(())
}
