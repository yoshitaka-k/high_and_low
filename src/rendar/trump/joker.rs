use ratatui::style::Color;
use ratatui::widgets::canvas::{Points, Painter, Shape};

/// ジョーカー図形を管理する構造体
pub struct Joker {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// ジョーカー図形を描画する
impl Shape for Joker {
    fn draw(&self, painter: &mut Painter) {
        Points::new(&[(self.x, self.y)], self.color).draw(painter);
    }
}
