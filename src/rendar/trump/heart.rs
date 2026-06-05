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
impl Shape for Heart {
    fn draw(&self, painter: &mut Painter) {
        let right_x = self.x + self.size;
        let right_y = self.y;
        let bottom_x = self.x;
        let bottom_y = self.y - self.size;
        let left_x = self.x - self.size;
        let left_y = self.y;

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
