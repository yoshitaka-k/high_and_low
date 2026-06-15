use crate::app::{App, GamePhase};

/// 結果画面で左クリックされたときの処理
pub(crate) fn handle_result_mouse_left(app: &mut App) {
    match app.current_phase {
        GamePhase::End => {
            app.advance_phase();
        }
        _ => {}
    }
}
