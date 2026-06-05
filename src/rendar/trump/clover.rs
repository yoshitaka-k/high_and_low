use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Line, Painter, Shape};

/// クローバー図形を管理する構造体
pub struct Clover {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// クローバー図形を描画する
impl Shape for Clover {
    fn draw(&self, painter: &mut Painter) {
        let right_x = self.x + self.size;
        let right_y = self.y;
        let bottom_x = self.x;
        let bottom_y = self.y - self.size;
        let left_x = self.x - self.size;
        let left_y = self.y;

        let radius = self.size * 0.45;

        // 上部の円
        Circle::new(
            left_x + self.size,
            left_y + self.size * 0.35,
            radius,
            self.color,
        )
        .draw(painter);
        // 左下の円
        Circle::new(
            left_x + self.size * 0.5,
            left_y - self.size * 0.35,
            radius,
            self.color,
        )
        .draw(painter);
        // 右下の円
        Circle::new(
            right_x - self.size * 0.5,
            right_y - self.size * 0.35,
            radius,
            self.color,
        )
        .draw(painter);

        // 茎（下向き三角）: ハートの逆パターン
        //   上辺の左右端 → 下の先端
        let stem_top_y = self.y - self.size * 0.7;
        let stem_half = self.size * 0.2;

        Line::new(
            self.x - stem_half,
            stem_top_y,
            bottom_x,
            bottom_y,
            self.color,
        )
        .draw(painter);
        Line::new(
            bottom_x,
            bottom_y,
            self.x + stem_half,
            stem_top_y,
            self.color,
        )
        .draw(painter);
    }
}
