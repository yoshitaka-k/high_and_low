use crate::app::App;

/// プレイフェーズに入ったときの処理
pub(crate) fn enter_playing(app: &mut App) {
    app.text.help = String::from(
        "Card strength: Ace > King > Queen > Jack > 10 > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2",
    );
}
