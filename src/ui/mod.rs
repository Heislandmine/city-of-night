use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::core::actions::{Action, TextMessage};
pub mod breaking;
pub mod home;
pub mod purchase_character;
pub mod user_view;

pub trait Component {
    fn render(&self, frame: &mut Frame);
    fn handle_key_pressed_event(&self, user_input: &String) -> Action;
}

pub fn render_output_message(message: &Option<TextMessage>, frame: &mut Frame, area: Rect) {
    let msg = match message {
        Some(text) => text.content.clone(),
        None => String::new(),
    };

    frame.render_widget(Paragraph::new(msg), area);
}
