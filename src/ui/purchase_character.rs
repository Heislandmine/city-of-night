use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    core::{actions::Action, game_data::CharactersAvailableForPurchase},
    ui::Component,
};

pub struct PurchaseCharacter {
    available_character_list: Vec<CharactersAvailableForPurchase>,
}

impl PurchaseCharacter {
    pub fn new(available_character_list: Option<Vec<CharactersAvailableForPurchase>>) -> Self {
        let character_list = match available_character_list {
            Some(e) => e,
            None => Vec::new(),
        };

        Self {
            available_character_list: character_list,
        }
    }

    pub fn set_available_character_list(&mut self, new_list: Vec<CharactersAvailableForPurchase>) {
        self.available_character_list = new_list;
    }
}

impl Component for PurchaseCharacter {
    fn render(&self, frame: &mut ratatui::Frame, string_inputted: &String) {
        let area = frame.area();

        let layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(3),
                Constraint::Max(3),
                Constraint::Max(1),
            ])
            .split(area);

        // コマンドエリアの描画
        let command_area_raws = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ],
        )
        .margin(1)
        .split(layouts[0]);
        frame.render_widget(
            Block::default()
                .border_type(BorderType::Plain)
                .borders(Borders::all()),
            layouts[0],
        );

        for (i, v) in self.available_character_list.iter().enumerate() {
            let name = v.display_name();
            let call_id = v.call_id();
            frame.render_widget(
                Paragraph::new(format!("[{call_id}]{name}")),
                command_area_raws[i],
            );
        }

        // footerコマンドエリアの描画
        frame.render_widget(Block::default().borders(Borders::all()), layouts[1]);
        let footer_command_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(1)
            .split(layouts[1]);

        frame.render_widget(Paragraph::new("[999]戻る"), footer_command_area[0]);

        // ユーザー入力表示エリア
        frame.render_widget(Paragraph::new(String::from(string_inputted)), layouts[2]);
    }

    fn handle_event(&self, user_input: &String) -> Action {
        let character_for_purchase = self
            .available_character_list
            .iter()
            .find(|e| *e.call_id() == *user_input);

        match character_for_purchase {
            Some(character) => Action::PurchaseCharacter(character.id()),
            None => Action::None,
        }
    }
}
