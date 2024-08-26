use ratatui::Frame;

use crate::core::actions::Action;
pub mod home;
pub mod purchase_character;

pub trait Component {
    fn render(&self, frame: &mut Frame, string_inputted: &String);
    fn handle_event(&self, user_input: &String) -> Action;
}
