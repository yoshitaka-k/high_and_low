use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::app::{App, CurrentScreen};
use crate::rendar::{
    title::render_title,
    header::render_header,
    footer::render_footer,
    content::render_content,
    end::render_end,
    popup::shuffle::render_shuffle,
    popup::exiting::render_exiting,
};

/// UI を描画する
pub fn render(frame: &mut Frame, app: &mut App) {
    // レイアウト定義
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ]);

    // 全体のレイアウト分割
    let [top, main, bottom] = frame.area().layout(&vertical);

    // header のレンダリング
    render_header(frame, top, app);

    match app.current_screen {
        CurrentScreen::Title => {
            app.back_screen = CurrentScreen::Title;
            render_title(frame, main);
        }
        CurrentScreen::Main => {
            app.back_screen = CurrentScreen::Main;
            render_content(frame, main, app);
        }
        CurrentScreen::End => {
            app.back_screen = CurrentScreen::End;
            render_end(frame, main, app);
        }
        CurrentScreen::Shuffle => {
            render_shuffle(frame, main, app);
        }
        CurrentScreen::Exiting => {
            render_exiting(frame);
        }
    }

    // footer のレンダリング
    render_footer(frame, bottom, app);
}
