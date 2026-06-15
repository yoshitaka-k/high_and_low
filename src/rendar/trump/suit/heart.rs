use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Line, Painter, Shape};

/// ハート図形を管理する構造体
pub struct Heart {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// ハート図形を描画する
///
/// `(x, y)` は外接矩形の中心を表す。
impl Shape for Heart {
    fn draw(&self, painter: &mut Painter) {
        let cx = self.x;
        let cy = self.y;
        let waist_y = cy + self.size * 0.16;

        let right_x = cx + self.size;
        let right_y = waist_y;
        let bottom_x = cx;
        let bottom_y = waist_y - self.size;
        let left_x = cx - self.size;
        let left_y = waist_y;

        let radius = self.size * 0.48;

        Line::new(right_x, right_y, bottom_x, bottom_y, self.color).draw(painter);
        Line::new(bottom_x, bottom_y, left_x, left_y, self.color).draw(painter);

        Circle::new(
            left_x + self.size * 0.45,
            left_y + self.size * 0.2,
            radius,
            self.color,
        )
        .draw(painter);

        Circle::new(
            right_x - self.size * 0.45,
            right_y + self.size * 0.2,
            radius,
            self.color,
        )
        .draw(painter);
    }
}
