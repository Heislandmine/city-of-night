use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::component::Component;

pub struct PurchaseCharacter {}

impl PurchaseCharacter {
    pub fn new() -> Self {
        Self {}
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
        frame.render_widget(Paragraph::new("[1]デモ子"), command_area_raws[0]);

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
}
