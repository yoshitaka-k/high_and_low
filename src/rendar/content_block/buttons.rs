use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::app::App;

///High and Low ボタンをレンタリングする
pub fn render_buttons(frame: &mut Frame, area: Rect, app: &mut App) {
    // ボタンのレイアウト
    let vertical =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1);
    let [top, bottom] = area.layout(&vertical);

    // ボタンスタイル
    let high_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let low_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));

    let high = Paragraph::new("high").block(high_block);
    let low = Paragraph::new("low").block(low_block);

    frame.render_widget(high, top);
    frame.render_widget(low, bottom);

    app.positions.set_high(top);
    app.positions.set_low(bottom);
}
