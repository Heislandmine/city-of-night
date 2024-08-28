use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    Frame,
};

use crate::core::{
    actions::PurchaseCharacterAction, contexts::RenderContext, game_controller::GameController,
    mode::ViewsMode,
};
use crate::{
    app_lib::tui::Tui,
    core::{
        actions::Action,
        game_data::{CharactersAvailableForPurchase, GameWorld, UserInventory},
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
                let mut action =
                    PurchaseCharacterAction::new(&mut self.game_world, &mut self.user_inventory);
                action.execute(id);
            }
            Action::None => {}
        }
    }

    pub fn handle_key_events(&mut self, context: RenderContext) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.quit(),
                        KeyCode::Char('n') => self.view.navigate(ViewsMode::PurchaseCharacter),
                        KeyCode::Char(c) => self.string_inputted.push(c),
                        KeyCode::Backspace => {
                            let _ = self.string_inputted.pop();
                        }
                        KeyCode::Enter => {
                            self.string_inputted.push('\n');
                            self.handle_events(context)
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    fn get_render_context(&self) -> RenderContext {
        let context = RenderContext::new(
            vec![CharactersAvailableForPurchase::new(
                "demo-ko".to_string(),
                "1".to_string(),
                "デモ子".to_string(),
                2000,
            )],
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

            self.handle_key_events(self.get_render_context())?;
        }

        tui.exit()?;

        Ok(())
    }
}
