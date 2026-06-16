use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    text::Text,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use figlet_rs::FIGlet;

/// タイトル画面をレンダリングする
pub fn render_title(frame: &mut Frame, area: Rect) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(6),
        Constraint::Length(7),
        Constraint::Fill(1),
    ]).spacing(1);
    let [_, title, message, _] = area.layout(&vertical);

    let figlet = FIGlet::standard().unwrap();
    let description = env!("CARGO_PKG_DESCRIPTION");
    let title_text = format!("{}", figlet.convert(description).unwrap());

    let title_paragraph = Paragraph::new(Text::from(title_text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    frame.render_widget(title_paragraph, title);

    let message_text = format!(
"---------------------------------------------------
Version: {}  |  License: {}
Starting {} Game Engine... 🚀
---------------------------------------------------\n
Click to continue",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_LICENSE"),
        env!("CARGO_PKG_NAME")
    );
    let message_paragraph = Paragraph::new(Text::from(message_text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    frame.render_widget(message_paragraph, message);
}
