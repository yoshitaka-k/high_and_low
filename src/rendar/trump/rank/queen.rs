use ratatui::style::Color;
use ratatui::widgets::canvas::{Line, Painter, Shape};

/// クイーンの図形を管理する構造体
pub struct Queen {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// クイーンの図形を描画する
impl Shape for Queen {
    fn draw(&self, painter: &mut Painter) {
        let cx = self.x;
        let cy = self.y;

        let left_top_x = cx - self.size;
        let left_top_y = cy + self.size;
        let right_top_x = cx + self.size * 0.7;
        let right_top_y = cy + self.size;

        let left_bottom_x = cx - self.size;
        let left_bottom_y = cy - self.size * 0.7;
        let right_bottom_x = cx + self.size * 0.7;
        let right_bottom_y = cy - self.size * 0.7;

        let line_right_bottom_x = cx + self.size;
        let line_right_bottom_y = cy - self.size;
        let line_center_middle_x = cx;
        let line_center_middle_y = cy;

        let lines = [
            Line::new(left_top_x, left_top_y, right_top_x, right_top_y, self.color),
            Line::new(right_top_x, right_top_y, right_bottom_x, right_bottom_y, self.color),
            Line::new(right_bottom_x, right_bottom_y, left_bottom_x, left_bottom_y, self.color),
            Line::new(left_bottom_x, left_bottom_y, left_top_x, left_top_y, self.color),

            Line::new(line_right_bottom_x, line_right_bottom_y, line_center_middle_x, line_center_middle_y, self.color),
        ];

        for line in &lines {
            line.draw(painter);
        }
    }
}
