use ratatui::layout::Position;

use crate::app::App;
use crate::game::PlayerChoice;

/// プレイ画面で左クリックされたときの処理
pub(crate) fn handle_playing(app: &mut App, mouse_pos: Position) {
    // 高い方を選択する
    if app.positions.high().contains(mouse_pos) {
        app.disp_text = String::from("High");
        app.game.set_choice(Some(PlayerChoice::High));
    }

    // 低い方を選択する
    if app.positions.low().contains(mouse_pos) {
        app.disp_text = String::from("Low");
        app.game.set_choice(Some(PlayerChoice::Low));
    }

    // エンターキーを押したら、次のフェーズへ進む
    if app.positions.enter().contains(mouse_pos) {
        if let Some(choice) = app.game.choice() {
            app.advance_phase();
        } else {
            app.disp_text = String::from("Please select a choice");
        }
    }
}
