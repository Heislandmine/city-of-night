use ratatui::widgets::Paragraph;

use crate::component::Component;

pub struct PurchaseCharacter {}

impl PurchaseCharacter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for PurchaseCharacter {
    fn render(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(Paragraph::new("奴隷購入画面"), frame.area())
    }
}
