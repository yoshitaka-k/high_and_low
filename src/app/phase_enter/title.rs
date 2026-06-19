use crate::app::{App, CurrentScreen};
use crate::constants::{DEFAULT_CREDITS};

/// タイトル画面に入ったときの処理
pub(crate) fn enter_title(app: &mut App) {
    app.turn = 1;

    app.player.set_credits(DEFAULT_CREDITS);
    app.game.set_bet(0);

    if app.current_screen != CurrentScreen::Title {
        app.current_screen = CurrentScreen::Title;
    }
}
