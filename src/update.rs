use ratatui::crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::app::{App, CurrentScreen, GamePhase};
use crate::handle::mouse_actions::{
    title_mouse_left::handle_title_mouse_left,
    main_mouse_left::handle_main_mouse_left,
    end_mouse_left::handle_end_mouse_left,
};
use crate::handle::tick::{
    setup::tick_setup,
    shuffle::tick_shuffle,
    deal::tick_deal,
    playing::tick_playing,
    result::tick_result,
    end::tick_end,
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
                CurrentScreen::Title => handle_title_mouse_left(app),
                CurrentScreen::Main => handle_main_mouse_left(app, mouse_event),
                CurrentScreen::End => handle_end_mouse_left(app),
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
        GamePhase::Playing => tick_playing(app),
        GamePhase::Result => tick_result(app),
        GamePhase::End => tick_end(app),
        _ => {}
    }

    // 次のフェーズへ進むタイミングなら、次のフェーズへ進む
    if app.game.tick() {
        app.advance_phase();
    }
}
