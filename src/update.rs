use crate::app::App;
use crate::app::CurrentScreen;
use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::app::GamePhase;
use crate::handle::mouse_actions::{
    main_mouse_left::handle_main_mouse_left,
    result_mouse_left::handle_result_mouse_left,
};
use crate::handle::tick::{
    deal::tick_deal,
    shuffle::tick_shuffle,
    setup::tick_setup,
    result::tick_result,
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
        // 左クリックされたときの処理
        MouseEventKind::Up(MouseButton::Left) => {
            match app.current_screen {
                CurrentScreen::Main => handle_main_mouse_left(app, mouse_event),
                CurrentScreen::Result => handle_result_mouse_left(app),
                _ => {}
            }
        }

        // 右クリックされたときの処理
        MouseEventKind::Up(MouseButton::Right) => {
            match app.current_screen {
                CurrentScreen::Exiting => {
                    app.current_screen = CurrentScreen::Main;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// 1 tick 進める
pub fn tick_update(app: &mut App) {
    match app.current_phase {
        GamePhase::Setup => tick_setup(app),
        GamePhase::Shuffle => tick_shuffle(app),
        GamePhase::Deal => tick_deal(app),
        GamePhase::Result => tick_result(app),
        _ => {}
    }

    // 次のフェーズへ進むタイミングなら、次のフェーズへ進む
    if app.game.tick() {
        app.advance_phase();
    }
}
