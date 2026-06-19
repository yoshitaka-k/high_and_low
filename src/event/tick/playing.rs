use crate::app::{App, GamePhase};

/// プレイフェーズの処理
pub(crate) fn tick_playing(app: &mut App) {
    if app.current_phase != GamePhase::Playing {
        return;
    }

    if *app.game.enter() {
        app.advance_phase();
    }
}
