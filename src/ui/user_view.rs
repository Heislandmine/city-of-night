use ratatui::Frame;

use crate::core::{contexts::RenderContext, mode::ViewsMode};

use super::{home::Home, purchase_character::PurchaseCharacter, Component};

pub struct UserView {
    current_view: ViewsMode,
}

impl UserView {
    pub fn new() -> Self {
        Self {
            current_view: ViewsMode::Home,
        }
    }

    pub fn render(&mut self, context: RenderContext, frame: &mut Frame) {
        match self.current_view {
            ViewsMode::Home => {
                let component = self.get_view(ViewsMode::Home, context);
                component.render(frame)
            }
            ViewsMode::PurchaseCharacter => {
                let component = self.get_view(ViewsMode::PurchaseCharacter, context);
                component.render(frame)
            }
        }
    }

    pub fn navigate(&mut self, to: ViewsMode) {
        self.current_view = to;
    }

    pub fn get_current_view(&mut self, context: RenderContext) -> Box<dyn Component> {
        match self.current_view {
            ViewsMode::Home => self.get_view(ViewsMode::Home, context),
            ViewsMode::PurchaseCharacter => self.get_view(ViewsMode::PurchaseCharacter, context),
        }
    }
    fn get_view(&mut self, view_id: ViewsMode, context: RenderContext) -> Box<dyn Component> {
        match view_id {
            ViewsMode::Home => Box::new(Home::new(context)),
            ViewsMode::PurchaseCharacter => Box::new(PurchaseCharacter::new(context)),
        }
    }
}
