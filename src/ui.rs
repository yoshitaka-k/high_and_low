use ratatui::{
    layout::{Constraint, Layout, Rect},
    symbols::Marker,
    text::Text,
    widgets::{
        Clear,
        canvas::{Canvas, Rectangle},
    },
    style::Color,
    Frame,
};

use crate::app::App;
use crate::trump::Suit;
use crate::rendar::trump::{
    diamond::Diamond,
    heart::Heart,
    spade::Spade,
    clover::Clover,
    joker::Joker,
};

/// UI を描画する
pub fn render(frame: &mut Frame, app: &mut App) {
    frame.render_widget(Clear, frame.area());

    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [area] = main.layout(&horizontal);

    let text = Text::from(if let Some(phase) = app.game().shuffle_phase_label() {
        format!("shuffling: {}", phase)
    } else {
        "shuffle finished".to_string()
    });

    frame.render_widget(text, top);

    render_canvas(frame, area, app);
}

/// キャンバスを描画する
fn render_canvas(frame: &mut Frame, area: Rect, app: &mut App) {
    let canvas = Canvas::default()
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&Rectangle::new(-25.0, -25.0, 50.0, 50.0, Color::White));
            ctx.print(-7.0, -22.0, app.text.to_string());
            ctx.layer();

            if let Some(card) = app.game().card() {
                match card.suit() {
                    Suit::Heart => {
                        ctx.draw(&Heart {
                            x: 0.0,
                            y: 5.0,
                            size: 15.0,
                            color: Color::White,
                        });
                    }
                    Suit::Diamond => {
                        ctx.draw(&Diamond {
                            x: 0.0,
                            y: 5.0,
                            size: 15.0,
                            color: Color::White,
                        });
                    }
                    Suit::Clover => {
                        ctx.draw(&Clover {
                            x: 0.0,
                            y: 5.0,
                            size: 15.0,
                            color: Color::White,
                        });
                    }
                    Suit::Spade => {
                        ctx.draw(&Spade {
                            x: 0.0,
                            y: 5.0,
                            size: 15.0,
                            color: Color::White,
                        });
                    }
                    Suit::Joker => {
                        ctx.draw(&Joker {
                            x: 0.0,
                            y: 5.0,
                            size: 15.0,
                            color: Color::White,
                        });
                    }
                }
            }
        });

    frame.render_widget(canvas, area);
}
