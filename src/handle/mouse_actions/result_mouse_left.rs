use crate::app::{App, CurrentScreen, GamePhase};

/// 結果画面で左クリックされたときの処理
pub(crate) fn handle_result_mouse_left(app: &mut App) {
    match app.current_phase {
        GamePhase::End => {
            app.current_screen = CurrentScreen::Main;
            app.current_phase = GamePhase::Setup;
        }
        _ => {}
    }
}
