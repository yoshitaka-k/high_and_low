use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::app::App;

/// ヘルプをレンダリングする
pub fn render_help(frame: &mut Frame, area: Rect, app: &App) {
    let help_block = Block::default()
        .title("Help")
        .padding(Padding::horizontal(1))
        .borders(Borders::ALL)
        .style(Style::default());

    let help = Paragraph::new(app.help_text.as_str())
    .block(help_block);

    frame.render_widget(help, area);
}
