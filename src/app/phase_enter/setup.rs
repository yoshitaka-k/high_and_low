use crate::app::App;
use crate::constants::WIN_STREAK_BONUS;

/// セットアップフェーズに入ったときの処理
pub(crate) fn enter_setup(app: &mut App) {
    app.text.win_bonus = format!("win bonus: {:.1}%", *app.player.win() as f32 * WIN_STREAK_BONUS * 100.0);

    app.turn = 1;
}
