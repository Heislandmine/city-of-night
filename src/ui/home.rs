use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, BorderType, Borders, Gauge, LineGauge, Paragraph},
    Frame,
};

use crate::{
    core::{actions::Action, character::Character, contexts::RenderContext, mode::ViewsMode},
    ui::Component,
};

pub struct Home {
    context: RenderContext,
}

impl Home {
    pub fn new(context: RenderContext) -> Self {
        Self { context }
    }

    fn render_main_ui(
        &self,
        frame: &mut Frame,
        string_inputted: &String,
        breaking_character: Option<Character>,
    ) {
        let area = frame.area();
        let display_breaking_character_info = match breaking_character {
            Some(e) => (3, e.max_hp(), e.current_hp(), e.display_name()),
            None => (0, 0, 0, String::new()),
        };

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(3 + display_breaking_character_info.0),
                Constraint::Min(3),
                Constraint::Max(1),
            ])
            .split(area);

        self.render_top_bar(frame, layout[0], display_breaking_character_info);

        frame.render_widget(Paragraph::new(String::from(string_inputted)), layout[2]);

        frame.render_widget(
            Block::new()
                .border_type(BorderType::Plain)
                .borders(Borders::all())
                .border_style(Style::default().fg(Color::White)),
            layout[1],
        );

        let command_area_raws = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ],
        )
        .split(layout[1]);

        let command_area_first_row = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ],
        )
        .margin(1)
        .split(command_area_raws[0]);

        frame.render_widget(
            Paragraph::new("[100]調教").centered(),
            command_area_first_row[0],
        );

        frame.render_widget(
            Paragraph::new("[101]奴隷購入").centered(),
            command_area_first_row[1],
        );

        frame.render_widget(
            Paragraph::new("[200]セーブ").centered(),
            command_area_first_row[2],
        );

        frame.render_widget(
            Paragraph::new("[300]ロード").centered(),
            command_area_first_row[3],
        );
    }

    fn render_top_bar(
        &self,
        frame: &mut Frame,
        area: Rect,
        breaking_character_info: (u16, u16, u16, String),
    ) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(3),
                Constraint::Min(breaking_character_info.0),
            ])
            .split(area);

        let top_bar_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .margin(1)
            .split(layout[0]);
        frame.render_widget(
            Block::new()
                .border_type(BorderType::Plain)
                .borders(Borders::all()),
            layout[0],
        );
        frame.render_widget(Paragraph::new("2124/8/5").centered(), top_bar_layout[0]);
        frame.render_widget(Paragraph::new("昼").centered(), top_bar_layout[1]);
        frame.render_widget(Paragraph::new("10日目").centered(), top_bar_layout[2]);
        frame.render_widget(Paragraph::new("残り200日").centered(), top_bar_layout[3]);

        let percent = if breaking_character_info.0 == 0 {
            0
        } else {
            ((breaking_character_info.2 as f32 / breaking_character_info.1 as f32) * 100.0) as u16
        };

        let character_info_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(1), Constraint::Min(1)])
            .split(layout[1]);

        frame.render_widget(
            Paragraph::new(breaking_character_info.3).style(Color::Green),
            character_info_area[0],
        );

        let hp_display_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Max(6), Constraint::Min(1)])
            .split(character_info_area[1]);

        frame.render_widget(Paragraph::new("体力: "), hp_display_area[0]);
        frame.render_widget(
            Gauge::default()
                .gauge_style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Black)
                        .add_modifier(Modifier::ITALIC),
                )
                .percent(percent),
            hp_display_area[1],
        )
    }
}

impl Component for Home {
    fn render(&self, frame: &mut Frame) {
        let msg = match &self.context.message {
            Some(s) => s.content.clone(),
            None => String::new(),
        };

        let breaking_character = self.context.breaking_character.clone();

        self.render_main_ui(frame, &msg, breaking_character);
    }

    fn handle_key_pressed_event(&self, user_input: &String) -> crate::core::actions::Action {
        if *user_input == String::from("101") {
            return Action::Navigate(ViewsMode::PurchaseCharacter);
        } else {
            return Action::None;
        }
    }
}
