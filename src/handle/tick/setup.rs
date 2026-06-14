use crate::app::App;

/// セットアップフェーズの処理
pub(crate) fn tick_setup(app: &mut App) {
    if !app.game.is_shuffling() {
        app.start();
        app.advance_phase();
    }
}
