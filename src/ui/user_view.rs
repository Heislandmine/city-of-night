use ratatui::Frame;

use crate::core::{actions::Action, contexts::RenderContext, mode::ViewsMode};

use super::{
    breaking::BreakingView,
    home::{Home, HomeRenderContext},
    purchase_character::PurchaseCharacter,
    Component,
};

pub struct UserView {
    current_view: ViewsMode,
    need_wait_enter_pressed: bool,
}

impl UserView {
    pub fn new() -> Self {
        Self {
            current_view: ViewsMode::Home,
            need_wait_enter_pressed: false,
        }
    }

    pub fn render(&mut self, context: RenderContext, frame: &mut Frame) {
        match &context.message {
            Some(e) => self.need_wait_enter_pressed = e.wait_enter_pressed,
            None => (),
        }

        match self.current_view {
            ViewsMode::Home => {
                let component = self.get_view(ViewsMode::Home, context);
                component.render(frame)
            }
            ViewsMode::PurchaseCharacter => {
                let component = self.get_view(ViewsMode::PurchaseCharacter, context);
                component.render(frame)
            }
            ViewsMode::Breaking => {
                let component = self.get_view(ViewsMode::Breaking, context);
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
            ViewsMode::Breaking => self.get_view(ViewsMode::Breaking, context),
        }
    }
    fn get_view(&mut self, view_id: ViewsMode, context: RenderContext) -> Box<dyn Component> {
        match view_id {
            ViewsMode::Home => Box::new(Home::new(HomeRenderContext::new(
                context.breaking_character,
                context.message,
            ))),
            ViewsMode::PurchaseCharacter => Box::new(PurchaseCharacter::new(context)),
            ViewsMode::Breaking => Box::new(BreakingView::new(context)),
        }
    }

    pub fn handle_key_pressed_event(
        &mut self,
        context: RenderContext,
        user_input: String,
    ) -> Action {
        if self.need_wait_enter_pressed {
            return Action::ResetMessage;
        }
        let current_view = self.get_current_view(context);

        current_view.handle_key_pressed_event(&user_input)
    }
}
