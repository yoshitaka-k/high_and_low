use crate::app::App;

/// ベット入力フェーズに入ったときの処理
pub(crate) fn enter_bet_input(app: &mut App) {
    app.text.bet_amount = format!("{} bets", *app.game.bet());
}
