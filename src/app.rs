use crate::game::Game;
use crate::rendar::block_position::BlockPosition;
use getset::{Getters, MutGetters, Setters};

/// 現在の画面を表す列挙型
#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Main,
    Result,
    Exiting,
}

#[derive(Debug, PartialEq)]
pub enum GamePhase {
    Setup,
    Shuffle,
    Deal,
    Play,
    Result,
    End,
}

/// アプリケーションの状態を表す構造体
#[derive(Getters, MutGetters, Setters)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_phase: GamePhase,
    pub positions: BlockPosition,
    pub should_quit: bool,
    pub game: Game,
    pub current: usize,
    pub turn: usize,
    pub header_text: String,
    pub footer_text: String,
    pub disp_text: String,
}

impl App {
    /// 新しいアプリケーションを作成する
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Main,
            current_phase: GamePhase::Setup,
            positions: BlockPosition::default(),
            should_quit: false,
            game: Game::new(),
            current: 0,
            turn: 1,
            header_text: String::new(),
            footer_text: String::new(),
            disp_text: String::new(),
        }
    }

    /// ゲームを開始する
    pub fn start(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.turn = 1;
        self.disp_text = String::new();
        self.game.start();
    }

    /// 次のフェーズへ進む
    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            GamePhase::Setup => GamePhase::Shuffle,
            GamePhase::Shuffle => GamePhase::Deal,
            GamePhase::Deal => GamePhase::Play,
            GamePhase::Play => GamePhase::Result,
            GamePhase::Result => GamePhase::End,
            GamePhase::End => {
                self.turn += 1;
                GamePhase::Deal
            },
        };
        self.on_phase_enter();
    }

    /// フェーズが変わったときの処理
    fn on_phase_enter(&mut self) {
        match self.current_phase {
            GamePhase::Setup => {
                self.turn = 1;
            },
            _ => {},
        }
    }

    /// アプリケーションを終了する
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
