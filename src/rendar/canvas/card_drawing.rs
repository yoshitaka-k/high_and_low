use ratatui::{
    Frame,
    layout::{Rect, Constraint, Layout, Alignment},
    style::Color,
    symbols::Marker,
    widgets::{Block, Borders, Paragraph, Padding},
    widgets::canvas::{Canvas, Context, Rectangle, Line},
};

use crate::trump::Card;
use crate::app::GamePhase;
use crate::rendar::content_block::field::CurrentCard;
use crate::rendar::canvas::{
    suit_drawing::suit_drawing,
    rank_drawing::rank_drawing,
};

/// 描画範囲のX軸の範囲
const X_BOUNDS: [f64; 2] = [-40.0, 80.0];
/// 描画範囲のY軸の範囲
const Y_BOUNDS: [f64; 2] = [-40.0, 70.0];

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
    phase: GamePhase,
) {
    // カードの外枠を描画する
    ctx.draw(&Rectangle::new(
        rectangle.x,
        rectangle.y,
        rectangle.width,
        rectangle.height,
        Color::Reset,
    ));

    ctx.layer();

    match current_card {
        CurrentCard::Dealer => {
            // スートを描画する
            suit_drawing(ctx, rectangle, card);
            ctx.layer();

            // ランクを描画する
            rank_drawing(ctx, rectangle, card);
        }
        CurrentCard::Player => {
            if card.is_some() && phase == GamePhase::Playing {
                let lines = [
                    Line::new(rectangle.x, rectangle.y, rectangle.x + rectangle.width, rectangle.y + rectangle.height, Color::White),
                ];

                for line in lines {
                    ctx.draw(&line);
                }
            } else if card.is_some() && (phase == GamePhase::Result || phase == GamePhase::End) {
                // スートを描画する
                suit_drawing(ctx, rectangle, card);
                ctx.layer();

                // ランクを描画する
                rank_drawing(ctx, rectangle, card);
            }
        }
    }
}

/// カードを1枚描画する
pub fn card_drawing(
    frame: &mut Frame,
    area: Rect,
    card: (&CardRectangle, CurrentCard, Option<&Card>),
    phase: GamePhase,
) {
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
    ]);
    let [name_area, canvas_area] = area.layout(&vertical);

    let name = Paragraph::new(match card.1 {
        CurrentCard::Dealer => "Dealer Card",
        CurrentCard::Player => "Player Card",
    })
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).padding(Padding::horizontal(1)));

    frame.render_widget(name, name_area);

    let canvas = Canvas::default()
        .x_bounds(X_BOUNDS)
        .y_bounds(Y_BOUNDS)
        .marker(Marker::Braille)
        .paint(|ctx| {
            paint_card(ctx, card.0, card.1, card.2, phase);
        });

    frame.render_widget(canvas, canvas_area);
}
