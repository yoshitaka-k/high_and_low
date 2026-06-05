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
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&Rectangle::new(8.0, area.height as f64 - 8.0, 10.0, 7.0, Color::White));
            ctx.print(10.0, area.height as f64 - 5.0, app.text.to_string());
        });

    frame.render_widget(canvas, area);
}
