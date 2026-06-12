use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::centered_rect;

/// Render the exit popup block
pub fn render_exiting(frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(5),
        Constraint::Fill(1),
    ]);
    let [_, middle, _] = frame.area().layout(&vertical);

    let horizontal = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(70),
        Constraint::Fill(1),
    ]);
    let [_, center, _] = middle.layout(&horizontal);

    let block = Block::default()
        .title("Quit Confirmation")
        .borders(Borders::ALL);
    frame.render_widget(block, center);

    let text = Text::styled(
        "Would you like to quit the app? (y/n)",
        Style::default().fg(Color::Red).bold(),
    );
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let area = centered_rect(70, 14, center);
    frame.render_widget(paragraph, area);
}
