use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::app::App;

///High and Low ボタンをレンタリングする
pub fn render_buttons(frame: &mut Frame, area: Rect, app: &mut App) {
    // ボタンのレイアウト
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(5),
        Constraint::Fill(1),
    ]);
    let [credits_disp, bet_disp, win_disp, choice_disp, high_btn, enter_btn, low_btn] = area.layout(&vertical);

    // 選択肢のブロックをレンダリング
    let disp_block = Block::new()
        .title("Choice")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    frame.render_widget(disp_block, choice_disp);

    // クレジットのブロックをレンダリング
    let credits_block = Block::new()
        .title("Credits")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let credits_paragraph = Paragraph::new(format!("{} credits", app.player.credits() - app.game.bet()))
        .block(credits_block);
    frame.render_widget(credits_paragraph, credits_disp);

    // ベットのブロックをレンダリング
    let bet_block = Block::new()
        .title("Bet")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let bet_paragraph = Paragraph::new(format!("{} bets", app.game.bet()))
        .block(bet_block);
    frame.render_widget(bet_paragraph, bet_disp);

    // 勝利のブロックをレンダリング
    let win_block = Block::new()
        .title("Win Streak")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let win_paragraph = Paragraph::new(format!(
        "{} win streak\n ({})",
        app.player.win(),
        app.text.win_bonus.as_str()
    ))
        .block(win_block);
    frame.render_widget(win_paragraph, win_disp);

    // 選択肢のテキストをレンダリング
    let disp_vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ]);
    let [_, disp_text_area, _] = choice_disp.layout(&disp_vertical);
    let choice_paragraph = Paragraph::new(app.text.disp.as_str())
        .alignment(Alignment::Center);
    frame.render_widget(choice_paragraph, disp_text_area);

    // ボタンスタイル
    let high_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::Green));
        let low_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::Yellow));
    let enter_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::Cyan));

    // ボタンをレンダリング
    let high_paraph = Paragraph::new("High").block(high_block);
    let low_paraph = Paragraph::new("Low").block(low_block);
    let enter_paraph = Paragraph::new("Enter").block(enter_block);

    // ボタンを描画
    frame.render_widget(high_paraph, high_btn);
    frame.render_widget(low_paraph, low_btn);
    frame.render_widget(enter_paraph, enter_btn);

    // ボタンの位置を設定
    app.positions.set_high(high_btn);
    app.positions.set_low(low_btn);
    app.positions.set_enter(enter_btn);
}
