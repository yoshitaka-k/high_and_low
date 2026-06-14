use crate::app::App;

/// シャッフルフェーズの処理
pub(crate) fn tick_shuffle(app: &mut App) {
    if !app.game.is_shuffling() {
        app.advance_phase();
    }
}
