use ratatui::style::Color;
use ratatui::widgets::canvas::{Points, Painter, Shape};

/// 1の図形を管理する構造体
pub struct One {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// 1の図形を描画する
/// 仮置きの図形で、実装していない
///
/// `(x, y)` は外接矩形の中心を表す。
impl Shape for One {
    fn draw(&self, painter: &mut Painter) {
        Points::new(&[(self.x, self.y), (self.x + self.size, self.y + self.size)], self.color).draw(painter);
    }
}
