use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    Frame,
};

use crate::core::{
    actions::{ActionResult, ActionStatus, PurchaseCharacterAction},
    contexts::RenderContext,
    game_controller::GameController,
    mode::ViewsMode,
};
use crate::{
    app_lib::tui::Tui,
    core::{
        actions::Action,
        game_data::{GameWorld, UserInventory},
    },
    ui::user_view::UserView,
};

pub struct App {
    should_quit: bool,
    string_inputted: String,
    game_controller: GameController,
    user_inventory: UserInventory,
    game_world: GameWorld,
    view: UserView,
}

impl App {
    pub fn new(
        user_inventory: UserInventory,
        game_world: GameWorld,
        game_controller: GameController,
        view: UserView,
    ) -> Self {
        Self {
            should_quit: false,
            user_inventory,
            game_world,
            game_controller,
            string_inputted: String::new(),
            view,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn render(&mut self, frame: &mut Frame, context: RenderContext) {
        self.view.render(context, frame)
    }

    fn handle_events(&mut self, context: RenderContext) {
        let current_view = self.view.get_current_view(context);

        let user_input = String::from(self.string_inputted.clone().trim());
        let action = current_view.handle_event(&user_input);

        match action {
            Action::PurchaseCharacter(id) => {
                self.game_controller
                    .handle_action(Action::PurchaseCharacter(id));
            }
            _ => {}
        }
    }

    pub fn handle_key_events(&mut self, context: RenderContext) -> io::Result<Action> {
        let mut action: Action = Action::None;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            action = Action::Quit;
                        }
                        KeyCode::Char('n') => {
                            action = Action::Navigate(ViewsMode::PurchaseCharacter);
                        }
                        KeyCode::Char(c) => {
                            action = Action::InputChar(c);
                        }
                        KeyCode::Backspace => {
                            action = Action::PressedBackspace;
                        }
                        KeyCode::Enter => {
                            self.string_inputted.push('\n');
                            self.handle_events(context);
                            action = Action::None;
                        }
                        _ => {
                            action = Action::None;
                        }
                    }
                }
            }
        }

        Ok(action)
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::None => ActionResult::new(ActionStatus::None, None),
            Action::Quit => {
                self.quit();
                ActionResult::new(ActionStatus::Success, None)
            }
            Action::Navigate(view_id) => {
                self.view.navigate(view_id);
                ActionResult::new(ActionStatus::Success, None)
            }
            Action::InputChar(c) => {
                self.string_inputted.push(c);
                ActionResult::new(ActionStatus::Success, None)
            }
            Action::PressedBackspace => {
                self.string_inputted.pop();
                ActionResult::new(ActionStatus::Success, None)
            }
            _ => self.game_controller.handle_action(action),
        };
    }

    fn get_render_context(&self) -> RenderContext {
        let context = RenderContext::new(
            self.game_controller
                .get_character_list_available_for_purchase(),
            self.string_inputted.clone(),
        );

        context
    }

    pub fn run(&mut self, mut tui: Tui) -> io::Result<()> {
        tui.enter()?;

        while !self.should_quit {
            tui.draw(|frame| {
                self.render(frame, self.get_render_context());
            })?;

            let action = self.handle_key_events(self.get_render_context())?;

            let action_result = self.handle_action(action);
        }

        tui.exit()?;

        Ok(())
    }
}
