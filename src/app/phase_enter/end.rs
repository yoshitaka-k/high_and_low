use ratatui::style::Color;

use crate::app::App;
use crate::game::{ResultLabel};

/// エンドフェーズに入ったときの処理
pub(crate) fn enter_end(app: &mut App) {
    app.text.result = app.game.result_label().to_string();

    app.color.result = match app.game.player_card_diff() {
        ResultLabel::Win => Color::Green,
        ResultLabel::Lose => Color::Red,
        _ => Color::Yellow,
    };
}
