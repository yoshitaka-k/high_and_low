use ratatui::layout::Position;

use crate::app::App;
use crate::game::PlayerChoice;

/// プレイ画面で左クリックされたときの処理
pub(crate) fn handle_playing(app: &mut App, mouse_pos: Position) {
    // 高い方を選択する
    if app.positions.high().contains(mouse_pos) {
        app.text.choice = String::from("High");
        app.game.set_choice(Some(PlayerChoice::High));
    }

    // 低い方を選択する
    if app.positions.low().contains(mouse_pos) {
        app.text.choice = String::from("Low");
        app.game.set_choice(Some(PlayerChoice::Low));
    }

    // 決定ボタンをクリックしたら、次のフェーズへ進む
    if app.positions.enter().contains(mouse_pos) {
        if app.game.choice().is_some() {
            app.game.set_enter(true);
        } else {
            app.text.choice = String::from("Please select a choice");
        }
    }
}
