use crate::game::Game;
use crate::rendar::block_position::BlockPosition;
use getset::{Getters, MutGetters, Setters};

/// 現在の画面を表す列挙型
#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Title,
    Main,
    End,
    Exiting,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GamePhase {
    Title,
    Setup,
    Shuffle,
    Deal,
    Playing,
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
    pub help_text: String,
    pub disp_text: String,
}

impl App {
    /// 新しいアプリケーションを作成する
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Title,
            current_phase: GamePhase::Title,
            positions: BlockPosition::default(),
            should_quit: false,
            game: Game::new(),
            current: 0,
            turn: 1,
            header_text: String::new(),
            footer_text: String::new(),
            help_text: String::new(),
            disp_text: String::new(),
        }
    }

    /// ゲームを開始する
    pub fn start(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.disp_text = String::new();
        self.help_text = String::new();
        self.game.start();
    }

    /// ゲームをリセットする
    pub fn reset(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.disp_text = String::new();
        self.help_text = String::new();
        self.game.reset();
    }

    /// 次のフェーズへ進む
    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            GamePhase::Title => GamePhase::Setup,
            GamePhase::Setup => GamePhase::Shuffle,
            GamePhase::Shuffle => GamePhase::Deal,
            GamePhase::Deal => GamePhase::Playing,
            GamePhase::Playing => GamePhase::Result,
            GamePhase::Result => GamePhase::End,
            GamePhase::End => {
                self.turn += 1;

                if self.game.deck().len() < 2 {
                    GamePhase::Setup
                } else {
                    self.reset();
                    GamePhase::Deal
                }

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
            GamePhase::Shuffle => {
                self.help_text = String::from("Shuffling the deck...");
            },
            GamePhase::Deal => {
                self.help_text = String::from("Dealing the cards...");
            },
            GamePhase::Playing => {
                self.help_text = String::from("Card strength: Ace > King > Queen > Jack > 10 > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2");
            },
            _ => {},
        }
    }

    /// アプリケーションを終了する
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
