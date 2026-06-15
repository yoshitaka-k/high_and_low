use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::app::{App, CurrentScreen, GamePhase};

/// フッターをレンダリングする
pub fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let horizontal =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1);
    let [left, right] = area.layout(&horizontal);

    let footer_left = footer_left_area(app);
    let footer_right = footer_right_area(app);

    // Render left and right messages
    frame.render_widget(footer_left, left);
    frame.render_widget(footer_right, right);
}

/// Renders the left footer paragraph based on the current screen mode.
fn footer_left_area(app: &App) -> Paragraph<'_> {
    // Current Navigation Text
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled("Game Playing", Style::default().fg(Color::Green).bold())
            }
            CurrentScreen::End => {
                Span::styled("Result", Style::default().fg(Color::Green).bold())
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting", Style::default().fg(Color::LightRed).bold())
            }
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            Span::styled(
                format!("{}", match app.current_phase {
                    GamePhase::Setup => "Setup Phase",
                    GamePhase::Shuffle => "Shuffle Phase",
                    GamePhase::Deal => "Deal Phase",
                    GamePhase::Playing => "Playing Phase",
                    GamePhase::Result => "Result Phase",
                    GamePhase::End => "End Phase",
                }),
                Style::default().fg(Color::DarkGray),
            )
        },
    ];

    Paragraph::new(Line::from(current_navigation_text)).block(
        Block::default()
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL),
    )
}

/// Renders the right footer paragraph based on the current screen mode.
fn footer_right_area(app: &App) -> Paragraph<'_> {
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Exiting => Span::styled(
                "(y) or (Enter) to yes, (n) or (Esc) to no",
                Style::default().fg(Color::Red).bold(),
            ),
            _ => Span::styled("(q) to exit", Style::default().fg(Color::Red).bold()),
        }
    };

    Paragraph::new(Line::from(current_keys_hint)).block(
        Block::default()
            .padding(Padding::horizontal(1))
            .borders(Borders::ALL),
    )
}
