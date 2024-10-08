use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::core::{actions::Action, character::Character, mode::ViewsMode};

use super::{render_contexts::BreakingRenderContext, render_output_message, Component};

pub struct BreakingView {
    context: BreakingRenderContext,
}

impl BreakingView {
    pub fn new(context: BreakingRenderContext) -> Self {
        Self { context }
    }

    fn render_character_info(&self, frame: &mut Frame, area: Rect, character_info: Character) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(1)])
            .margin(1)
            .split(area);

        frame.render_widget(
            Paragraph::new(format!("キャラクター名: {}", character_info.display_name())),
            layout[0],
        );

        let display_hp_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Max(6), Constraint::Min(1)])
            .split(layout[1]);

        frame.render_widget(Block::new().borders(Borders::all()), area);

        let percent =
            ((character_info.current_hp() as f32 / character_info.max_hp() as f32) * 100.0) as u16;
        frame.render_widget(Paragraph::new("体力: "), display_hp_area[0]);
        frame.render_widget(
            Gauge::default()
                .gauge_style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Black)
                        .add_modifier(Modifier::ITALIC),
                )
                .percent(percent),
            display_hp_area[1],
        )
    }
}
impl Component for BreakingView {
    fn render(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(7),
                Constraint::Min(1),
                Constraint::Max(3),
                Constraint::Max(1),
            ])
            .split(frame.area());

        self.render_character_info(frame, layout[0], self.context.breaking_character.clone());

        // コマンドエリアの描画
        frame.render_widget(Block::new().borders(Borders::all()), layout[1]);
        let command_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .margin(1)
            .split(layout[1]);
        frame.render_widget(Paragraph::new("[1]愛撫"), command_area[0]);

        // フッターコマンドエリアの描画
        frame.render_widget(Block::new().borders(Borders::all()), layout[2]);
        let footer_command_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .margin(1)
            .split(layout[2]);
        frame.render_widget(Paragraph::new("[999]終了"), footer_command_area[0]);

        // テキストエリアの描画
        render_output_message(&self.context.message, frame, layout[3]);
    }
    fn handle_key_pressed_event(&self, user_input: &String) -> crate::core::actions::Action {
        if *user_input == String::from("999") {
            return Action::Navigate(ViewsMode::Home);
        }
        Action::None
    }
}
