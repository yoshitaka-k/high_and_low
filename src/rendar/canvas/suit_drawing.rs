use ratatui::{
    layout::Rect,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Context, Rectangle},
    },
    style::Color,
    Frame,
};

use crate::trump::{Card, Suit};
use crate::rendar::trump::{
    diamond::Diamond,
    heart::Heart,
    spade::Spade,
    clover::Clover,
    joker::Joker,
};

pub struct SuitRectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

fn paint_card(ctx: &mut Context<'_>, rectangle: &SuitRectangle, label: String, card: Option<&Card>) {
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
            Suit::Heart => ctx.draw(&Heart { x: suit_x, y: suit_y, size: suit_size, color }),
            Suit::Diamond => ctx.draw(&Diamond { x: suit_x, y: suit_y, size: suit_size, color }),
            Suit::Clover => ctx.draw(&Clover { x: suit_x, y: suit_y, size: suit_size, color }),
            Suit::Spade => ctx.draw(&Spade { x: suit_x, y: suit_y, size: suit_size, color }),
            Suit::Joker => ctx.draw(&Joker { x: suit_x, y: suit_y, size: suit_size, color }),
        }
    }
}

/// ディーラーとプレイヤーのカードを描画する
pub fn suit_drawing(
    frame: &mut Frame,
    area: Rect,
    dealer: (&SuitRectangle, String, Option<&Card>),
    player: (&SuitRectangle, String, Option<&Card>),
) {
    let canvas = Canvas::default()
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .marker(Marker::Braille)
        .paint(|ctx| {
            paint_card(ctx, dealer.0, dealer.1.clone(), dealer.2);
            paint_card(ctx, player.0, player.1.clone(), player.2);
        });

    frame.render_widget(canvas, area);
}
