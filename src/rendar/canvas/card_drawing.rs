use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::canvas::{Canvas, Context, Rectangle},
};


use crate::trump::Card;
use crate::rendar::canvas::suit_drawing::suit_drawing;

/// カードの矩形定義デフォルト値
pub const CARD_RECT: CardRectangle = CardRectangle {
    x: -25.0,
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

    suit_drawing(ctx, rectangle, card);
}

/// カードを1枚描画する
pub fn card_drawing(
    frame: &mut Frame,
    area: Rect,
    card: (&CardRectangle, String, Option<&Card>),
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
