use ratatui::crossterm::event::MouseEvent;
use ratatui::layout::Position;

use crate::app::{App, GamePhase};
use crate::event::actions::bet_input::handle_bet_input;

/// ベット入力画面で右クリックされたときの処理
pub(crate) fn handle_bet_input_mouse_left(app: &mut App, mouse_event: MouseEvent) {
    let mouse_pos = Position::new(mouse_event.column, mouse_event.row);

    match app.current_phase {
        GamePhase::BetInput => handle_bet_input(app, mouse_pos),
        _ => {}
    }
}
