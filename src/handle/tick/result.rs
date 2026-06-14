use crate::app::App;
use crate::app::{CurrentScreen, GamePhase};

/// 結果フェーズの処理
pub(crate) fn tick_result(app: &mut App) {
    match app.current_phase {
        GamePhase::Result => {
            app.current_screen = CurrentScreen::Result;
            app.advance_phase();
        }
        _ => {}
    }
}
