use crate::app::App;
use crate::app::GamePhase;
use crate::constants::TICK_RATE_MILLIS;

/// 結果画面を表示した後少し待機する（tick数に変換したもの）
const RESULT_PAUSE_TICKS: u8 = (1500 / TICK_RATE_MILLIS) as u8;

/// 結果フェーズの処理
pub(crate) fn tick_result(app: &mut App) {
    if app.current_phase != GamePhase::Result {
        return;
    }

    // プレイヤーカードを表示した後少し待機
    app.game.schedule_phase_advance(RESULT_PAUSE_TICKS);
}
