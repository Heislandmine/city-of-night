use ratatui::Frame;
pub mod home;
pub mod purchase_character;

pub trait Component {
    fn render(&self, frame: &mut Frame, string_inputted: &String);
}
