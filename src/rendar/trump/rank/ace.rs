use ratatui::style::Color;
use ratatui::widgets::canvas::{Line, Painter, Shape};

/// エースの図形を管理する構造体
pub struct Ace {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// エースの図形を描画する
impl Shape for Ace {
    fn draw(&self, painter: &mut Painter) {
        let cx = self.x;
        let cy = self.y;

        let top_x = cx;
        let top_y = cy + self.size;
        let right_bottom_x = cx + self.size;
        let right_bottom_y = cy - self.size;
        let left_bottom_x = cx - self.size;
        let left_bottom_y = cy - self.size;

        let left_middle_x = cx - self.size * 0.7;
        let left_middle_y = cy;
        let right_middle_x = cx + self.size * 0.7;
        let right_middle_y = cy;

        let lines = [
            Line::new(top_x, top_y, right_bottom_x, right_bottom_y, self.color),
            Line::new(top_x, top_y, left_bottom_x, left_bottom_y, self.color),
            Line::new(left_middle_x, left_middle_y, right_middle_x, right_middle_y, self.color),
        ];

        for line in &lines {
            line.draw(painter);
        }
    }
}
