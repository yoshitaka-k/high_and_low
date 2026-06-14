use ratatui::style::Color;
use ratatui::widgets::canvas::{Circle, Line, Painter, Shape};

/// スペード図形を管理する構造体
pub struct Spade {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Color,
}

/// スペード図形を描画する
///
/// `(x, y)` は外接矩形の中心を表す。
impl Shape for Spade {
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

        let radius = self.size * 0.48;

        Line::new(left_x, left_y, top_x, top_y, self.color).draw(painter);
        Line::new(top_x, top_y, right_x, right_y, self.color).draw(painter);

        Circle::new(
            left_x + self.size * 0.45,
            left_y - self.size * 0.2,
            radius,
            self.color,
        ).draw(painter);
        Circle::new(
            right_x - self.size * 0.45,
            right_y - self.size * 0.2,
            radius,
            self.color,
        ).draw(painter);

        // 茎（下向き三角）: ハートの逆パターン
        //   上辺の左右端 → 下の先端
        let stem_top_y = cy - self.size * 0.7;
        let stem_half = self.size * 0.2;

        Line::new(
            cx - stem_half,
            stem_top_y,
            bottom_x,
            bottom_y,
            self.color,
        )
        .draw(painter);
        Line::new(
            bottom_x,
            bottom_y,
            cx + stem_half,
            stem_top_y,
            self.color,
        )
        .draw(painter);    }
}
