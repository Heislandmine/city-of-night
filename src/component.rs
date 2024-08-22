use ratatui::Frame;

pub trait Component {
    fn render(&self, frame: &mut Frame, string_inputted: &String);
}
