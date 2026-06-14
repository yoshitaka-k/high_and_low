use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    text::Text,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use figlet_rs::FIGlet;

use crate::app::App;

/// 結果をレンダリングする
pub fn render_result(frame: &mut Frame, area: Rect, app: &mut App) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(7),
        Constraint::Length(3),
        Constraint::Fill(1),
    ]).spacing(1);
    let [_, result, message, _] = area.layout(&vertical);

    let figlet = FIGlet::standard().unwrap();
    let result_label = app.game.result_label();
    let result_text = format!("{}", figlet.convert(result_label).unwrap());

    let result_paragraph = Paragraph::new(Text::from(result_text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    frame.render_widget(result_paragraph, result);

    let message_text = format!(
        "--------------------------------\n{result_label}\nClick to continue\n--------------------------------",
    );
    let message_paragraph = Paragraph::new(Text::from(message_text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    frame.render_widget(message_paragraph, message);

}
