use crate::app::App;

/// シャッフルフェーズに入ったときの処理
pub(crate) fn enter_shuffle(app: &mut App) {
    app.shuffle_spinner_ticks = 0;
    app.text.help = String::from("Shuffling the deck...");
}
