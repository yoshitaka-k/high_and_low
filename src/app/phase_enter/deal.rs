use crate::app::{App, CurrentScreen};

/// ディールフェーズに入ったときの処理
pub(crate) fn enter_deal(app: &mut App) {
    app.text.help = String::from("Dealing the cards...");

    if app.current_screen != CurrentScreen::Main {
        app.current_screen = CurrentScreen::Main;
    }
}
