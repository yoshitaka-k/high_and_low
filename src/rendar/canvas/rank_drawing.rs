use ratatui::{
    style::Color,
    widgets::canvas::{Context},
};

use crate::rendar::trump::rank::{
    ace::Ace,
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
use crate::trump::card_enum::rank::Rank;
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
            Rank::Ace => ctx.draw(&Ace {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Two => ctx.draw(&Two {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Three => ctx.draw(&Three {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Four => ctx.draw(&Four {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Five => ctx.draw(&Five {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Six => ctx.draw(&Six {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Seven => ctx.draw(&Seven {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Eight => ctx.draw(&Eight {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Nine => ctx.draw(&Nine {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Ten => ctx.draw(&Ten {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Jack => ctx.draw(&Jack {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::Queen => ctx.draw(&Queen {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
            Rank::King => ctx.draw(&King {
                x: rank_x,
                y: rank_y,
                size: rank_size,
                color,
            }),
        }
    }
}
