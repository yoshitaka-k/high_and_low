use crate::app::App;

/// ディールフェーズに入ったときの処理
pub(crate) fn enter_deal(app: &mut App) {
    app.text.help = String::from("Dealing the cards...");
}
