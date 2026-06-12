use crate::game::Game;
use crate::rendar::block_position::BlockPosition;
use crate::trump::Card;
use getset::{Getters, MutGetters, Setters};

/// 現在の画面を表す列挙型
#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Exiting,
}

/// アプリケーションの状態を表す構造体
#[derive(Getters, MutGetters, Setters)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub positions: BlockPosition,
    pub should_quit: bool,
    pub game: Game,
    pub current: usize,
    pub dealer_card: Option<Card>,
    pub player_card: Option<Card>,
    pub header_text: &'static str,
    pub footer_text: &'static str,
}

impl App {
    /// 新しいアプリケーションを作成する
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Main,
            positions: BlockPosition::default(),
            should_quit: false,
            game: Game::new(),
            current: 0,
            dealer_card: None,
            player_card: None,
            header_text: "",
            footer_text: "",
        }
    }

    /// ゲームを開始する
    pub fn start(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.current = 0;
        self.dealer_card = None;
        self.player_card = None;
        self.game.start();
    }

    /// 次のフェーズへ進む
    pub fn advance_phase(&mut self) {
        println!("advance_phase");
        self.on_phase_enter();
    }

    /// フェーズが変わったときの処理
    fn on_phase_enter(&mut self) {
        println!("on_phase_enter");
    }

    /// アプリケーションを終了する
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
