use ratatui::Frame;

use crate::core::actions::Action;
pub mod home;
pub mod purchase_character;
pub mod user_view;

pub trait Component {
    fn render(&self, frame: &mut Frame);
    fn handle_event(&self, user_input: &String) -> Action;
}
