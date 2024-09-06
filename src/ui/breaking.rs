use ratatui::{widgets::Paragraph, Frame};

use crate::core::{actions::Action, contexts::RenderContext};

use super::Component;

pub struct BreakingView {
    context: RenderContext,
}

impl BreakingView {
    pub fn new(context: RenderContext) -> Self {
        Self { context }
    }
}
impl Component for BreakingView {
    fn render(&self, frame: &mut Frame) {
        frame.render_widget(Paragraph::new("調教画面"), frame.area())
    }
    fn handle_key_pressed_event(&self, user_input: &String) -> crate::core::actions::Action {
        Action::None
    }
}
