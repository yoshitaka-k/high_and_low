use ratatui::{
    style::Color,
    widgets::canvas::{Context},
};

use crate::rendar::trump::suit::{
    clover::Clover, diamond::Diamond, heart::Heart, joker::Joker, spade::Spade,
};
use crate::trump::Card;
use crate::trump::card::Suit;
use crate::rendar::canvas::card_drawing::CardRectangle;

/// スートを描画する
pub(crate) fn suit_drawing(
    ctx: &mut Context<'_>,
    rectangle: &CardRectangle,
    card: Option<&Card>,
) {

    // 左上辺りにスートを描画する
    let suit_x = rectangle.x + rectangle.width / 3.0;
    let suit_y = rectangle.y + rectangle.height / 1.4;
    let suit_size = rectangle.width.min(rectangle.height) * 0.21;

    if let Some(card) = card {
        let color = match card.suit() {
            Suit::Heart => Color::Red,
            Suit::Diamond => Color::Red,
            Suit::Clover => Color::DarkGray,
            Suit::Spade => Color::DarkGray,
            Suit::Joker => Color::Reset,
        };

        match card.suit() {
            Suit::Heart => ctx.draw(&Heart {
                x: suit_x,
                y: suit_y,
                size: suit_size,
                color,
            }),
            Suit::Diamond => ctx.draw(&Diamond {
                x: suit_x,
                y: suit_y,
                size: suit_size,
                color,
            }),
            Suit::Clover => ctx.draw(&Clover {
                x: suit_x,
                y: suit_y,
                size: suit_size,
                color,
            }),
            Suit::Spade => ctx.draw(&Spade {
                x: suit_x,
                y: suit_y,
                size: suit_size,
                color,
            }),
            Suit::Joker => ctx.draw(&Joker {
                x: suit_x,
                y: suit_y,
                size: suit_size,
                color,
            }),
        }
    }
}
