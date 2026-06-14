use ratatui::layout::Rect;
use getset::{Getters, Setters};

#[derive(Default, Getters, Setters)]
pub struct BlockPosition {
    #[getset(get = "pub", set = "pub")]
    high: Rect,
    #[getset(get = "pub", set = "pub")]
    low: Rect,
    #[getset(get = "pub", set = "pub")]
    enter: Rect,
}
