use crate::app::{App, CurrentScreen, GamePhase};

/// エンドフェーズの処理
pub(crate) fn tick_end(app: &mut App) {
    if app.current_phase != GamePhase::End {
        return;
    }

    if app.current_screen != CurrentScreen::End
        && app.current_screen != CurrentScreen::Exiting {
        app.current_screen = CurrentScreen::End;
    }
}
