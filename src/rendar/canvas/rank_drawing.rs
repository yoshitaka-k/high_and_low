use ratatui::{
    style::Color,
    widgets::canvas::{Context},
};

use crate::rendar::trump::rank::{
    ace::Ace,
    zero::Zero,
    two::Two,
    three::Three,
    four::Four,
    five::Five,
    six::Six,
    seven::Seven,
    eight::Eight,
    nine::Nine,
    ten::Ten,
    jack::Jack,
    queen::Queen,
    king::King,
};
use crate::trump::Card;
use crate::rendar::canvas::card_drawing::CardRectangle;

/// ランクを描画する
pub(crate) fn rank_drawing(
    ctx: &mut Context<'_>,
    rectangle: &CardRectangle,
    card: Option<&Card>,
) {

    // 右下辺りにランクを描画する
    let rank_x = rectangle.x + rectangle.width - rectangle.width / 3.5;
    let rank_y = rectangle.y + rectangle.height - rectangle.height / 1.4;
    let rank_size = rectangle.width.min(rectangle.height) * 0.15;

    if let Some(card) = card {
        let color = Color::Reset;

        match card.rank() {
            1 => ctx.draw(&Ace {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            2 => ctx.draw(&Two {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            3 => ctx.draw(&Three {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            4 => ctx.draw(&Four {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            5 => ctx.draw(&Five {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            6 => ctx.draw(&Six {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            7 => ctx.draw(&Seven {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            8 => ctx.draw(&Eight {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            9 => ctx.draw(&Nine {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            10 => ctx.draw(&Ten {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            11 => ctx.draw(&Jack {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            12 => ctx.draw(&Queen {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            13 => ctx.draw(&King {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            _ => ctx.draw(&Zero {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
        }
    }
}
