use ratatui::style::Color;
use ratatui::widgets::canvas::{Line, Painter, Shape};

/// 8の図形を管理する構造体
pub struct Eight {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// 8の図形を描画する
impl Shape for Eight {
    fn draw(&self, painter: &mut Painter) {
        let cx = self.x;
        let cy = self.y;

        let left_top_x = cx - self.size;
        let left_top_y = cy + self.size;
        let right_top_x = cx + self.size;
        let right_top_y = cy + self.size;

        let left_bottom_x = cx - self.size;
        let left_bottom_y = cy - self.size;
        let right_bottom_x = cx + self.size;
        let right_bottom_y = cy - self.size;

        let right_middle_x = cx + self.size;
        let right_middle_y = cy;
        let left_middle_x = cx - self.size;
        let left_middle_y = cy;

        let lines = [
            Line::new(left_top_x, left_top_y, right_top_x, right_top_y, self.color),
            Line::new(right_top_x, right_top_y, right_bottom_x, right_bottom_y, self.color),
            Line::new(right_bottom_x, right_bottom_y, left_bottom_x, left_bottom_y, self.color),
            Line::new(left_bottom_x, left_bottom_y, left_top_x, left_top_y, self.color),
            Line::new(left_middle_x, left_middle_y, right_middle_x, right_middle_y, self.color),
        ];

        for line in &lines {
            line.draw(painter);
        }
    }
}
