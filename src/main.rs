use app::App;
use character::Character;
use component::Component;
use home::Home;
use purchase_character::PurchaseCharacter;
use std::{
    collections::HashMap,
    io::{self},
};
use tui::Tui;

mod app;
mod character;
mod character_sheet;
mod component;
mod home;
mod purchase_character;
mod tui;

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
