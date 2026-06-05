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
impl Shape for Diamond {
    fn draw(&self, painter: &mut Painter) {
        let top_x = self.x;
        let top_y = self.y + self.size;
        let right_x = self.x + self.size;
        let right_y = self.y;
        let bottom_x = self.x;
        let bottom_y = self.y - self.size;
        let left_x = self.x - self.size;
        let left_y = self.y;

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
