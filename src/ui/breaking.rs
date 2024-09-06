use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::core::{actions::Action, character::Character, contexts::RenderContext};

use super::Component;

pub struct BreakingView {
    context: RenderContext,
}

impl BreakingView {
    pub fn new(context: RenderContext) -> Self {
        Self { context }
    }

    fn render_character_info(&self, frame: &mut Frame, area: Rect, character_info: Character) {
        let display_hp_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Max(6), Constraint::Min(1)])
            .margin(1)
            .split(area);
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
                Constraint::Max(6),
                Constraint::Min(1),
                Constraint::Max(1),
            ])
            .split(frame.area());

        self.render_character_info(
            frame,
            layout[0],
            Character::new("tr".to_string(), "デモ子".to_string(), 2000),
        )
    }
    fn handle_key_pressed_event(&self, user_input: &String) -> crate::core::actions::Action {
        Action::None
    }
}
