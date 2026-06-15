use ratatui::{
    widgets::canvas::{Painter, Shape},
    style::Color,
    widgets::canvas::{Line, Circle},
};

pub struct BackDesign {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

/// カードの裏面を描画する
impl Shape for BackDesign {
    fn draw(&self, painter: &mut Painter) {
        let cx = self.x;
        let cy = self.y;

        let line1_top_left_x = cx - self.size;
        let line1_top_left_y = cy - self.size;
        let line1_bottom_right_x = cx + self.size;
        let line1_bottom_right_y = cy + self.size;

        let line2_top_right_x = cx + self.size;
        let line2_top_right_y = cy - self.size;
        let line2_bottom_left_x = cx - self.size;
        let line2_bottom_left_y = cy + self.size;

        let color = Color::Green;

        let lines = [
            Line::new(line1_top_left_x, line1_top_left_y, line1_bottom_right_x, line1_bottom_right_y, color),
            Line::new(line2_top_right_x, line2_top_right_y, line2_bottom_left_x, line2_bottom_left_y, color),
        ];

        for line in lines {
            line.draw(painter);
        }

        Circle::new(
            cx,
            cy,
            self.size,
            Color::Red,
        ).draw(painter);

        Circle::new(
            cx,
            cy,
            self.size * 0.75,
            Color::Yellow,
        ).draw(painter);
    }
}
