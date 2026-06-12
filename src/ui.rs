use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::app::{App, CurrentScreen};
use crate::rendar::{
    content::render_content, footer::render_footer, header::render_header,
    popup::exiting::render_exiting,
};

/// UI を描画する
pub fn render(frame: &mut Frame, app: &mut App) {
    // レイアウト定義
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .spacing(1);

    // 全体のレイアウト分割
    let [top, main, bottom] = frame.area().layout(&vertical);

    // header のレンダリング
    render_header(frame, top, app);

    match app.current_screen {
        CurrentScreen::Main => {
            render_content(frame, main, app);
        }
        CurrentScreen::Exiting => {
            render_exiting(frame);
        }
    }

    // footer のレンダリング
    render_footer(frame, bottom, app);
}
