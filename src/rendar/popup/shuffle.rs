use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{app::App, centered_rect, constants::SHUFFLE_SPINNER};

/// Render the exit popup block
pub fn render_shuffle(frame: &mut Frame, area: Rect, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(5),
        Constraint::Fill(1),
    ]);
    let [_, middle, _] = area.layout(&vertical);

    let horizontal = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(70),
        Constraint::Fill(1),
    ]);
    let [_, center, _] = middle.layout(&horizontal);

    let block = Block::default()
        .title("Message Popup")
        .borders(Borders::ALL);
    frame.render_widget(block, center);

    let spinner = SHUFFLE_SPINNER[app.shuffle_spinner_frame()];
    let text = Text::styled(
        format!("Shuffling the deck... {}", spinner),
        Style::default(),
    );
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let area = centered_rect(70, 14, center);
    frame.render_widget(paragraph, area);
}
