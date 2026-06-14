use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::app::App;

/// ヘッダーをレンダリングする
pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let header_block = Block::default()
        .padding(Padding::horizontal(1))
        .borders(Borders::ALL)
        .style(Style::default());

    let header = Paragraph::new(Text::styled(
        &app.header_text,
        Style::default().fg(Color::Green).bold(),
    ))
    .block(header_block);

    frame.render_widget(header, area);
}
