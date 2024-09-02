use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    Frame,
};

use crate::core::{
    actions::{ActionResult, ActionStatus, KeyPressedEvent, TextMessage},
    contexts::RenderContext,
    game_controller::GameController,
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
    last_action_result: ActionResult,
    game_controller: GameController,
    game_world: GameWorld,
    view: UserView,
}

impl App {
    pub fn new(game_world: GameWorld, game_controller: GameController, view: UserView) -> Self {
        Self {
            should_quit: false,
            game_world,
            last_action_result: ActionResult::none(),
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

    pub fn handle_key_events(&mut self) -> io::Result<KeyPressedEvent> {
        let mut event: KeyPressedEvent = KeyPressedEvent::None;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => {
                            event = KeyPressedEvent::KeyPressed(c);
                        }
                        KeyCode::Backspace => {
                            event = KeyPressedEvent::PressedBackspace;
                        }
                        KeyCode::Enter => {
                            event = KeyPressedEvent::PressedEnter(self.string_inputted.clone());
                            self.string_inputted.clear();
                        }
                        _ => {
                            event = KeyPressedEvent::None;
                        }
                    }
                }
            }
        }

        Ok(event)
    }

    fn handle_key_pressed_event(
        &mut self,
        key_pressed_event: KeyPressedEvent,
        context: RenderContext,
    ) -> Action {
        match key_pressed_event {
            KeyPressedEvent::KeyPressed('q') => Action::Quit,
            KeyPressedEvent::KeyPressed(c) => Action::PushToCharacterFromInputtedString(c),
            KeyPressedEvent::PressedBackspace => Action::PopOneCharacterFromInputtedString,
            KeyPressedEvent::PressedEnter(user_input) => {
                self.view.handle_key_pressed_event(context, user_input)
            }
            KeyPressedEvent::None => Action::None,
        }
    }
    fn handle_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::None => ActionResult::none(),
            Action::Quit => {
                self.quit();
                ActionResult::success(None)
            }
            Action::Navigate(view_id) => {
                self.view.navigate(view_id);
                ActionResult::success(None)
            }
            Action::PushToCharacterFromInputtedString(c) => {
                self.string_inputted.push(c);
                ActionResult::success(Some(TextMessage::new(self.string_inputted.clone(), false)))
            }
            Action::PopOneCharacterFromInputtedString => {
                self.string_inputted.pop();
                ActionResult::success(Some(TextMessage::new(self.string_inputted.clone(), false)))
            }
            Action::ResetMessage => {
                self.string_inputted.clear();
                ActionResult::success(Some(TextMessage::new(self.string_inputted.clone(), false)))
            }
            _ => self.game_controller.handle_action(action),
        }
    }

    fn get_render_context(&self) -> RenderContext {
        let context = RenderContext::new(
            self.game_controller
                .get_character_list_available_for_purchase(),
            self.last_action_result.message.clone(),
        );

        context
    }

    pub fn run(&mut self, mut tui: Tui) -> io::Result<()> {
        tui.enter()?;

        while !self.should_quit {
            tui.draw(|frame| {
                self.render(frame, self.get_render_context());
            })?;

            let key_pressed_event = self.handle_key_events()?;

            let action =
                self.handle_key_pressed_event(key_pressed_event, self.get_render_context());

            let action_result = self.handle_action(action);

            match action_result.status {
                ActionStatus::None => (),
                _ => self.last_action_result = action_result,
            }
        }

        tui.exit()?;

        Ok(())
    }
}
