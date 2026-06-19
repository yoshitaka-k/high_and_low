use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

/// Render the bet input popup block
pub fn render_bet_input(frame: &mut Frame, _area: Rect, app: &mut App) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Min(1),
        Constraint::Fill(1),
    ]);
    let [_, middle, _] = frame.area().layout(&vertical);

    let horizontal = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(70),
        Constraint::Fill(1),
    ]);
    let [_, center, _] = middle.layout(&horizontal);

    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ]);
    let [bet_text, bet_area, bet_btns] = center.layout(&vertical);

    // 枠線の表示
    let block = Block::default()
        .borders(Borders::ALL);
    frame.render_widget(block.clone(), bet_area);

    // テキストを表示
    let text_paragraph = Paragraph::new("Please enter your bet amount")
        .alignment(Alignment::Center);
    frame.render_widget(text_paragraph, bet_text);

    // ベット
    let input_block = Block::default()
        .title("Inpu bet amount")
        .borders(Borders::ALL);
    let bet_paragraph = Paragraph::new(app.text.bet_amount.clone())
        .alignment(Alignment::Center)
        .block(input_block);
    frame.render_widget(bet_paragraph, bet_area);

    let bet_btns_horizontal = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ]);
    let [bet_up_btn, bet_down_btn, bet_enter_btn] = bet_btns.layout(&bet_btns_horizontal);

    // ベットアップボタンを表示
    let bet_up_paragraph = Paragraph::new("Bet Up")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green))
        .block(block.clone());
    frame.render_widget(bet_up_paragraph, bet_up_btn);
    app.positions.set_bet_up(bet_up_btn);

    // ベットダウンボタンを表示
    let bet_down_paragraph = Paragraph::new("Bet Down")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow))
        .block(block.clone());
    frame.render_widget(bet_down_paragraph, bet_down_btn);
    app.positions.set_bet_down(bet_down_btn);

    // ベットエンターボタンを表示
    let bet_enter_paragraph = Paragraph::new("Bet Enter")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan))
        .block(block.clone());
    frame.render_widget(bet_enter_paragraph, bet_enter_btn);
    app.positions.set_bet_enter(bet_enter_btn);
}
