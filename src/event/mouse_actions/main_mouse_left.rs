use ratatui::crossterm::event::MouseEvent;
use ratatui::layout::Position;

use crate::app::{App, GamePhase};
use crate::event::actions::playing::handle_playing;

/// メイン画面で左クリックされたときの処理
pub(crate) fn handle_main_mouse_left(app: &mut App, mouse_event: MouseEvent) {
    let mouse_pos = Position::new(mouse_event.column, mouse_event.row);

    match app.current_phase {
        GamePhase::Playing => handle_playing(app, mouse_pos),
        _ => {}
    }
}
