use ratatui::style::Color;

use crate::constants::WIN_STREAK_BONUS;
use crate::app::App;
use crate::game::{ResultLabel};

/// エンドフェーズに入ったときの処理
pub(crate) fn enter_end(app: &mut App) {
    // 勝利数を取得する
    let max_win = *app.player.max_win();
    let win = match app.game.player_card_diff() {
        ResultLabel::Win => app.player.win() + 1,
        ResultLabel::Lose => 0,
        _ => 0,
    };

    // 勝利数を更新して、最大勝利数を更新する
    app.player.set_win(win);
    if win > max_win {
        app.player.set_max_win(win);
    }

    let bet = *app.game.bet();
    let bonus = win as f32 * WIN_STREAK_BONUS;
    let result = match app.game.player_card_diff() {
        ResultLabel::Win => bet + (bet as f32 * bonus) as i32,
        ResultLabel::Lose => -bet,
        _ => 0,
    };
    app.player.credit_add(result);

    app.text.bet_result = format!("{}", result);
    app.text.credits = format!("{}", *app.player.credits());

    // 勝敗結果に応じて、テキストと色を更新する
    app.text.result = app.game.result_label().to_string();
    app.color.result = match app.game.player_card_diff() {
        ResultLabel::Win => Color::Green,
        ResultLabel::Lose => Color::Red,
        _ => Color::Yellow,
    };

    // クレジットが0以下になったらゲームオーバーにする
    if *app.player.credits() <= 0 {
        app.text.result = String::from("Game Over");
        app.color.result = Color::Red;
    }
}
