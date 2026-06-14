use ratatui::{
    Frame,
    layout::{Layout, Constraint, Rect},
};

use crate::app::App;
use crate::rendar::{
    canvas::suit_drawing::{CARD_RECT, suit_drawing},
};

/// フィールドをレンダリングする
pub fn render_field(frame: &mut Frame, area: Rect, app: &App) {
    let horizontal = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]).spacing(1);
    let [dealer, player] = area.layout(&horizontal);

    // ディーラーとプレイヤーのカードのラベル
    let dealer_label = app
        .game.dealer_card()
        .as_ref()
        .map(|c| format!("dealer: {c}"))
        .unwrap_or_else(|| "dealer".to_string());

    let player_label = app
        .game.player_card()
        .as_ref()
        .map(|_c| format!("player: ??"))
        .unwrap_or_else(|| "player".to_string());

    // ディーラーとプレイヤーのカードを描画する
    suit_drawing(
        frame,
        dealer,
        (&CARD_RECT, dealer_label, app.game.dealer_card().as_ref()),
    );

    suit_drawing(
        frame,
        player,
        (&CARD_RECT, player_label, app.game.player_card().as_ref()),
    );
}
