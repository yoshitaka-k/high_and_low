use crate::app::App;
use crate::app::CurrentScreen;
use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

/// キーイベントを処理する関数
pub fn key_update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Exiting => match key_event.code {
            KeyCode::Enter | KeyCode::Char('y') => app.should_quit = true,
            KeyCode::Esc | KeyCode::Char('n') => app.current_screen = CurrentScreen::Main,
            _ => {}
        },
        _ => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                app.current_screen = CurrentScreen::Exiting;
            }
            _ => {}
        },
    }
}

/// マウスイベントを処理する関数
pub fn mouse_update(app: &mut App, mouse_event: MouseEvent) {
    match mouse_event.kind {
        MouseEventKind::Up(MouseButton::Right) => app.quit(),
        _ => {}
    }
}

/// 1 tick 進める
pub fn tick_update(app: &mut App) {
    if !app.game.is_shuffling() {
        app.current = (app.current + 1) % 2;

        if let Some(card) = app.game.deck_mut().draw() {
            if app.current == 0 {
                app.dealer_card = Some(card);
            } else {
                app.player_card = Some(card);
            }
        } else {
            app.start();
        }
    }

    // 次のフェーズへ進むタイミングなら、次のフェーズへ進む
    if app.game.tick() {
        app.advance_phase();
    }
}
