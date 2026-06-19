use ratatui::layout::Position;

use crate::constants::DEFAULT_BET;
use crate::app::App;

/// ベット入力画面で左クリックされたときの処理
pub(crate) fn handle_bet_input(app: &mut App, mouse_pos: Position) {
    // ベットアップボタンをクリックしたら、ベット額を増やす
    if app.positions.bet_up().contains(mouse_pos) {
        if app.game.bet() + DEFAULT_BET > *app.player.credits() {
            app.text.bet_amount = format!("{} bets", app.player.credits());
            app.game.set_bet(*app.player.credits());
            return;
        }

        app.text.bet_amount = format!("{} bets", app.game.bet() + DEFAULT_BET);
        app.game.set_bet(app.game.bet() + DEFAULT_BET);
    }

    // ベットダウンボタンをクリックしたら、ベット額を減らす
    if app.positions.bet_down().contains(mouse_pos) {
        if app.game.bet() - DEFAULT_BET < 0 {
            app.text.bet_amount = String::from("0 bets");
            app.game.set_bet(0);
            return;
        }

        app.text.bet_amount = format!("{} bets", app.game.bet() - DEFAULT_BET);
        app.game.set_bet(app.game.bet() - DEFAULT_BET);
    }

    // ベットエンターボタンをクリックしたら、ベット額を確定する
    if app.positions.bet_enter().contains(mouse_pos) {
        if *app.game.bet() <= 0 {
            return;
        }

        app.advance_phase();
    }
}
