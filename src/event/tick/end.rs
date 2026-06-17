use ratatui::style::Color;

use crate::app::App;
use crate::app::{CurrentScreen, GamePhase};
use crate::game::ResultLabel;

/// エンドフェーズの処理
pub(crate) fn tick_end(app: &mut App) {
    if app.current_phase != GamePhase::End {
        return;
    }

    if app.current_screen != CurrentScreen::End
        && app.current_screen != CurrentScreen::Exiting {

        app.color.result = match app.game.player_card_diff() {
            ResultLabel::Win => Color::Green,
            ResultLabel::Lose => Color::Red,
            _ => Color::Yellow,
        };

        app.current_screen = CurrentScreen::End;
    }
}
