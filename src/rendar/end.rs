use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    text::Text,
    style::Style,
    widgets::Paragraph,
    Frame,
};
use figlet_rs::FIGlet;

use crate::app::App;

/// エンド画面をレンダリングする
pub fn render_end(frame: &mut Frame, area: Rect, app: &mut App) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(5),
        Constraint::Length(8),
        Constraint::Fill(1),
    ]);
    let [_, result, message, _] = area.layout(&vertical);

    let figlet = FIGlet::standard().unwrap();
    let result_text = format!("{}", figlet.convert(app.text.result.as_str()).unwrap());

    let result_paragraph = Paragraph::new(Text::from(result_text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.color.result));

    frame.render_widget(result_paragraph, result);

    let message_text = format!(
"\n----------------------------------------
Get bets: {}
to total {} credits.
----------------------------------------\n
Click to continue",
        app.text.bet_result,
        app.text.credits,
    );
    let message_paragraph = Paragraph::new(Text::from(message_text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(app.color.result));

    frame.render_widget(message_paragraph, message);

}
