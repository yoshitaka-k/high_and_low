use ratatui::layout::Position;

use crate::constants::DEFAULT_BET;
use crate::app::App;

/// ベット入力画面で右クリックされたときの処理
pub(crate) fn handle_bet_input(app: &mut App, mouse_pos: Position) {
    let bet = *app.game.bet();
    let credits = *app.player.credits();

    // ベットアップボタンをクリックしたら、ベット額を増やす
    if app.positions.bet_up().contains(mouse_pos) {
        let new_bet = bet + DEFAULT_BET;

        if new_bet > credits {
            app.text.bet_amount = format!("{} bets / credits: {}", bet, credits - bet);
            app.game.set_bet(credits);
            return;
        }

        app.text.bet_amount = format!("{} bets / credits: {}", new_bet, credits - new_bet);
        app.game.set_bet(new_bet);
    }

    // ベットダウンボタンをクリックしたら、ベット額を減らす
    if app.positions.bet_down().contains(mouse_pos) {
        let new_bet = bet - DEFAULT_BET;

        if new_bet < 0 {
            app.text.bet_amount = format!("0 bets / credits: {}", credits - bet);
            app.game.set_bet(0);
            return;
        }
        app.text.bet_amount = format!("{} bets / credits: {}", new_bet, credits - new_bet);
        app.game.set_bet(new_bet);
    }

    // ベットエンターボタンをクリックしたら、ベット額を確定する
    if app.positions.bet_enter().contains(mouse_pos) {
        if bet <= 0 {
            return;
        }

        app.game.set_input_bet(bet);

        app.advance_phase();
    }
}
