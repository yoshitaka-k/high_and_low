use crate::app::{App, CurrentScreen, GamePhase};

/// ベット入力フェーズの処理
pub(crate) fn tick_bet_input(app: &mut App) {
    if app.current_phase != GamePhase::BetInput {
        return;
    }

    if app.current_screen != CurrentScreen::BetInput
        && app.current_screen != CurrentScreen::Exiting {
        app.current_screen = CurrentScreen::BetInput;
    }
}
