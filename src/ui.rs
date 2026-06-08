use ratatui::{
    layout::{Constraint, Layout},
    text::Text,
    Frame,
};

use crate::app::App;
use crate::rendar::canvas::{
    suit_drawing::{SuitRectangle, suit_drawing},
};

const CARD_RECT: SuitRectangle = SuitRectangle {
    x: -25.0,
    y: 0.0,
    width: 50.0,
    height: 50.0,
};

/// UI を描画する
pub fn render(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ]).spacing(1);
    let horizontal = Layout::horizontal([
        Constraint::Percentage(100)
    ]).spacing(1);

    let [top, main, _bottom] = frame.area().layout(&vertical);
    let [area] = main.layout(&horizontal);

    let text = Text::from(if let Some(phase) = app.game.shuffle_phase_label() {
        format!("shuffling: {}", phase)
    } else {
        "shuffle finished".to_string()
    });

    frame.render_widget(text, top);

    let dealer_rect = SuitRectangle { y: -30.0, ..CARD_RECT };
    let player_rect = SuitRectangle { y: 40.0, ..CARD_RECT };

    let dealer_label = app.dealer_card.as_ref()
        .map(|c| format!("dealer: {c}"))
        .unwrap_or_else(|| "dealer".to_string());
    let player_label = app.player_card.as_ref()
        .map(|c| format!("player: {c}"))
        .unwrap_or_else(|| "player".to_string());

    suit_drawing(
        frame,
        area,
        (&dealer_rect, dealer_label, app.dealer_card.as_ref()),
        (&player_rect, player_label, app.player_card.as_ref()),
    );
}
