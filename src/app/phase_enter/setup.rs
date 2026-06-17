use crate::app::App;

/// セットアップフェーズに入ったときの処理
pub(crate) fn enter_setup(app: &mut App) {
    app.turn = 1;
}
