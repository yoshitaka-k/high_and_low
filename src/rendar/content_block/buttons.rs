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
    let vertical =
        Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Fill(1),
        ]);
        // .spacing(1);
    let [top, disp, enter, bottom] = area.layout(&vertical);

    // ディスプレイスタイル
    let disp_block = Block::new()
        .title("Disp")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    frame.render_widget(disp_block, disp);

    let disp_vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ]);
    let [_, disp_text_area, _] = disp.layout(&disp_vertical);

    let disp_pagraph = Paragraph::new(app.text.disp.as_str()).alignment(Alignment::Center);

    frame.render_widget(disp_pagraph, disp_text_area);

    // ボタンスタイル
    let high_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::Green));
    let enter_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::Cyan));
    let low_block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::Yellow));

    // ボタンをレンダリング
    let high_paraph = Paragraph::new("High").block(high_block);
    let low_paraph = Paragraph::new("Low").block(low_block);
    let enter_paraph = Paragraph::new("Enter").block(enter_block);

    // ボタンを描画
    frame.render_widget(high_paraph, top);
    frame.render_widget(enter_paraph, enter);
    frame.render_widget(low_paraph, bottom);

    // ボタンの位置を設定
    app.positions.set_high(top);
    app.positions.set_enter(enter);
    app.positions.set_low(bottom);
}
