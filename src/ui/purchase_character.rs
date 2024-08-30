use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    core::{actions::Action, contexts::RenderContext, mode::ViewsMode},
    ui::Component,
};

pub struct PurchaseCharacter {
    context: RenderContext,
}

impl PurchaseCharacter {
    pub fn new(context: RenderContext) -> Self {
        Self { context }
    }
}

impl Component for PurchaseCharacter {
    fn render(&self, frame: &mut ratatui::Frame) {
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

        for (i, v) in self
            .context
            .character_list_available_for_purchase
            .iter()
            .enumerate()
        {
            let name = v.display_name();
            let call_id = v.call_id();
            let price = v.price();
            frame.render_widget(
                Paragraph::new(format!("[{call_id}]{name} {price}G")),
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
        let msg = match &self.context.message {
            Some(s) => s.content.clone(),
            None => "".to_string(),
        };
        frame.render_widget(Paragraph::new(msg), layouts[2]);
    }

    fn handle_key_pressed_event(&self, user_input: &String) -> Action {
        if *user_input == String::from("999") {
            return Action::Navigate(ViewsMode::Home);
        }

        let character_for_purchase = self
            .context
            .character_list_available_for_purchase
            .iter()
            .find(|e| *e.call_id() == *user_input);

        match character_for_purchase {
            Some(character) => Action::PurchaseCharacter(character.id()),
            None => Action::None,
        }
    }
}
