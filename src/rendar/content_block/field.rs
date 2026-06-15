use ratatui::{
    Frame,
    layout::{Layout, Constraint, Rect},
};

use crate::app::App;
use crate::rendar::{
    canvas::card_drawing::{CARD_RECT, card_drawing},
    content_block::help::render_help,
};

#[derive(PartialEq, Clone, Copy)]
pub enum CurrentCard {
    Dealer,
    Player,
}

/// フィールドをレンダリングする
pub fn render_field(frame: &mut Frame, area: Rect, app: &App) {
    // フィールドのレイアウト
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
    ]);
    let [field, help] = area.layout(&vertical);

    render_help(frame, help, app);

    // ディーラーとプレイヤーのカードを描画するためのレイアウト
    let horizontal = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]).spacing(1);
    let [dealer, player] = field.layout(&horizontal);

    // ディーラーとプレイヤーのカードを描画する
    card_drawing(
        frame,
        dealer,
        (&CARD_RECT, CurrentCard::Dealer, app.game.dealer_card().as_ref()),
        app.current_phase,
    );

    card_drawing(
        frame,
        player,
        (&CARD_RECT, CurrentCard::Player, app.game.player_card().as_ref()),
        app.current_phase,
    );
}
