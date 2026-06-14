use ratatui::style::Color;
use ratatui::widgets::canvas::{Points, Painter, Shape};

/// なし図形を管理する構造体
pub struct Zero {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// なし図形を描画する
///
/// `(x, y)` は図形の中心を表す。
impl Shape for Zero {
    fn draw(&self, painter: &mut Painter) {
        Points::new(&[(self.x, self.y)], self.color).draw(painter);
    }
}
