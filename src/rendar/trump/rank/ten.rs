use ratatui::style::Color;
use ratatui::widgets::canvas::{Line, Painter, Shape};

/// 10の図形を管理する構造体
pub struct Ten {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// 10の図形を描画する
impl Shape for Ten {
    fn draw(&self, painter: &mut Painter) {
        let cx = self.x;
        let cy = self.y;

        let one_left_top_x = cx - self.size;
        let one_left_top_y = cy + self.size;
        let one_left_bottom_x = cx - self.size;
        let one_left_bottom_y = cy - self.size;

        let zero_left_top_x = cx - self.size * 0.5;
        let zero_left_top_y = cy + self.size;
        let zero_right_top_x = cx + self.size;
        let zero_right_top_y = cy + self.size;

        let zero_left_bottom_x = cx - self.size * 0.5;
        let zero_left_bottom_y = cy - self.size;
        let zero_right_bottom_x = cx + self.size;
        let zero_right_bottom_y = cy - self.size;

        let lines = [
            Line::new(one_left_top_x, one_left_top_y, one_left_bottom_x, one_left_bottom_y, self.color),

            Line::new(zero_left_top_x, zero_left_top_y, zero_right_top_x, zero_right_top_y, self.color),
            Line::new(zero_right_top_x, zero_right_top_y, zero_right_bottom_x, zero_right_bottom_y, self.color),
            Line::new(zero_right_bottom_x, zero_right_bottom_y, zero_left_bottom_x, zero_left_bottom_y, self.color),
            Line::new(zero_left_bottom_x, zero_left_bottom_y, zero_left_top_x, zero_left_top_y, self.color),
        ];

        for line in &lines {
            line.draw(painter);
        }
    }
}
