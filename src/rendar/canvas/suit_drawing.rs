use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::canvas::{Canvas, Context, Rectangle},
};

use crate::rendar::trump::{
    clover::Clover, diamond::Diamond, heart::Heart, joker::Joker, spade::Spade,
};
use crate::trump::{Card, Suit};

/// カードの矩形定義デフォルト値
pub const CARD_RECT: SuitRectangle = SuitRectangle {
    x: -25.0,
    y: 0.0,
    width: 50.0,
    height: 50.0,
};

/// カードの矩形定義
pub struct SuitRectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// カードを描画する
fn paint_card(
    ctx: &mut Context<'_>,
    rectangle: &SuitRectangle,
    label: String,
    card: Option<&Card>,
) {
    let rank_x = rectangle.x + rectangle.width / 2.7;
    let rank_y = rectangle.y + 3.0;

    ctx.draw(&Rectangle::new(
        rectangle.x,
        rectangle.y,
        rectangle.width,
        rectangle.height,
        Color::White,
    ));
    ctx.print(rank_x, rank_y, label);

    ctx.layer();

    let suit_x = rectangle.x + rectangle.width / 3.0;
    let suit_y = rectangle.y + rectangle.height / 1.5;
    let suit_size = rectangle.width.min(rectangle.height) * 0.23;

    if let Some(card) = card {
        let color = Color::White;
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

/// カードを1枚描画する
pub fn suit_drawing(
    frame: &mut Frame,
    area: Rect,
    card: (&SuitRectangle, String, Option<&Card>),
) {
    let canvas = Canvas::default()
        .x_bounds([-90.0, 90.0])
        .y_bounds([-90.0, 90.0])
        .marker(Marker::Braille)
        .paint(|ctx| {
            paint_card(ctx, card.0, card.1.clone(), card.2);
        });

    frame.render_widget(canvas, area);
}
