use crate::app::{App, GamePhase};

/// タイトル画面で左クリックされたときの処理
pub(crate) fn handle_title_mouse_left(app: &mut App) {
    if app.current_phase != GamePhase::Title {
        return;
    }

    app.advance_phase();
}
