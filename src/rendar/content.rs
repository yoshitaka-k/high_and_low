use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::rendar::content_block::field::render_field;
use crate::{app::App, rendar::content_block::buttons::render_buttons};

/// コンテンツをレンダリングする
pub fn render_content(frame: &mut Frame, area: Rect, app: &mut App) {
    let horizontal =
        Layout::horizontal([Constraint::Percentage(100), Constraint::Length(25)]).spacing(1);
    let [main, button] = area.layout(&horizontal);

    // フィールドのレンダリング
    render_field(frame, main, app);

    // ボタンのレンダリング
    render_buttons(frame, button, app);
}
