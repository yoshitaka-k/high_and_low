use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::app::App;

/// ヘッダーをレンダリングする
pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let outer_block = Block::default()
        .padding(Padding::horizontal(1))
        .borders(Borders::ALL);
    frame.render_widget(outer_block, area);

    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ]);
    let [_, inner_area, _] = area.layout(&vertical);

    let horizontal = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(25),
    ]);
    let [left_area, right_area] = inner_area.layout(&horizontal);

    // ヘッダー内のブロック
    let inner_block = Block::default()
        .padding(Padding::horizontal(2));

    // ヘッダーテキストのレンダリング
    let header_text = Paragraph::new(Text::styled(
        &app.header_text,
        Style::default().fg(Color::Green).bold(),
    )).block(inner_block.clone());
    frame.render_widget(header_text, left_area);

    // Ticker / Fps のレンダリング
    let ticker_fps = Paragraph::new(Text::styled(
            app.ticker_fps.display(),
            Style::default().fg(Color::DarkGray)
        ))
        .alignment(Alignment::Right)
        .block(inner_block.clone());
    frame.render_widget(ticker_fps, right_area);
}
