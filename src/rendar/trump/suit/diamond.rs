use ratatui::style::Color;
use ratatui::widgets::canvas::{Line, Painter, Shape};

/// ダイアモンド図形を管理する構造体
pub struct Diamond {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// ダイアモンド図形を描画する
///
/// `(x, y)` は外接矩形の中心を表す。
impl Shape for Diamond {
    fn draw(&self, painter: &mut Painter) {
        let cx = self.x;
        let cy = self.y;

        let top_x = cx;
        let top_y = cy + self.size;
        let right_x = cx + self.size;
        let right_y = cy;
        let bottom_x = cx;
        let bottom_y = cy - self.size;
        let left_x = cx - self.size;
        let left_y = cy;

        let lines = [
            Line::new(left_x, left_y, top_x, top_y, self.color),
            Line::new(top_x, top_y, right_x, right_y, self.color),
            Line::new(right_x, right_y, bottom_x, bottom_y, self.color),
            Line::new(bottom_x, bottom_y, left_x, left_y, self.color),
        ];
        for line in &lines {
            line.draw(painter);
        }
    }
}
