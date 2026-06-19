use crate::app::App;
use crate::constants::WIN_STREAK_BONUS;

/// プレイフェーズに入ったときの処理
pub(crate) fn enter_playing(app: &mut App) {
    let creadits = *app.player.credits();
    let bet = *app.game.bet();

    if creadits - bet < 0 {
        app.text.credits = format!("{} credits", 0);
        app.game.set_bet(creadits);
    } else {
        app.text.credits = format!("{} credits", creadits - bet);
    }

    if creadits < bet {
        app.text.bet = format!("{} bets", creadits);
    } else {
        app.text.bet = format!("{} bets", bet);
    }

    app.text.win_bonus = format!("win bonus: {:.1}%", *app.player.win() as f32 * WIN_STREAK_BONUS * 100.0);

    app.text.help = String::from(
        "Card strength: Ace > King > Queen > Jack > 10 > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2",
    );
}
