use crate::app::App;
use crate::app::{CurrentScreen, GamePhase};

/// 結果フェーズの処理
pub(crate) fn tick_result(app: &mut App) {
    match app.current_phase {
        GamePhase::Result => {
            // 結果画面を表示した後少し待機
            std::thread::sleep(std::time::Duration::from_millis(1000));

            app.current_screen = CurrentScreen::Result;
            app.advance_phase();
        }
        _ => {}
    }
}
