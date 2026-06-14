use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::canvas::{Canvas, Context, Rectangle, Line},
};

use crate::trump::Card;
use crate::rendar::content_block::field::CurrentCard;
use crate::rendar::canvas::{
    suit_drawing::suit_drawing,
    rank_drawing::rank_drawing,
};

/// カードの矩形定義デフォルト値
pub const CARD_RECT: CardRectangle = CardRectangle {
    x: 0.0,
    y: 0.0,
    width: 50.0,
    height: 50.0,
};

/// カードの矩形定義
pub struct CardRectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// カードを描画する
fn paint_card(
    ctx: &mut Context<'_>,
    rectangle: &CardRectangle,
    current_card: CurrentCard,
    card: Option<&Card>,
) {
    // カードの外枠を描画する
    ctx.draw(&Rectangle::new(
        rectangle.x,
        rectangle.y,
        rectangle.width,
        rectangle.height,
        Color::White,
    ));

    ctx.layer();

    if current_card == CurrentCard::Dealer {
        // スートを描画する
        suit_drawing(ctx, rectangle, card);
        ctx.layer();

        // ランクを描画する
        rank_drawing(ctx, rectangle, card);

    } else if current_card == CurrentCard::Player {
        let lines = [
            Line::new(rectangle.x, rectangle.y, rectangle.x + rectangle.width, rectangle.y + rectangle.height, Color::White),
        ];

        for line in lines {
            ctx.draw(&line);
        }
    }
}

/// カードを1枚描画する
pub fn card_drawing(
    frame: &mut Frame,
    area: Rect,
    card: (&CardRectangle, CurrentCard, Option<&Card>),
) {
    let canvas = Canvas::default()
        .x_bounds([-90.0, 90.0])
        .y_bounds([-90.0, 90.0])
        .marker(Marker::Braille)
        .paint(|ctx| {
            paint_card(ctx, card.0, card.1, card.2);
        });

    frame.render_widget(canvas, area);
}
