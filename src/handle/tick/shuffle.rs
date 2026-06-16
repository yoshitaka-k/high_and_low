use crate::app::{App, CurrentScreen};

/// シャッフルフェーズの処理
pub(crate) fn tick_shuffle(app: &mut App) {
    app.current_screen = CurrentScreen::Shuffle;
    app.advance_shuffle_spinner();

    if !app.game.is_shuffling() {
        app.current_screen = CurrentScreen::Main;
        app.advance_phase();
    }
}
