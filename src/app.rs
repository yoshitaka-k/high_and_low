use crate::components::TickerFps;
use crate::game::Game;
use crate::rendar::block_position::BlockPosition;
use getset::{Getters, MutGetters, Setters};

/// 現在の画面を表す列挙型
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CurrentScreen {
    Title,
    Main,
    End,
    Shuffle,
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

/// アプリケーションのテキストを表す構造体
#[derive(Default, Getters, MutGetters, Setters)]
pub struct AppText {
    pub header: String,
    pub footer: String,
    pub help: String,
    pub disp: String,
    pub result: String,
}

impl AppText {
    pub fn new() -> Self {
        Self {
            header: String::new(),
            footer: String::new(),
            help: String::new(),
            disp: String::new(),
            result: String::new(),
        }
    }
}

/// アプリケーションの状態を表す構造体
pub struct App {
    pub current_screen: CurrentScreen,
    pub back_screen: CurrentScreen,
    pub current_phase: GamePhase,
    pub positions: BlockPosition,
    pub should_quit: bool,
    pub game: Game,
    pub current: usize,
    pub turn: usize,
    
    pub text: AppText,

    pub ticker_fps: TickerFps,

    /// 次のフェーズへ進むタイミングをスケジュールする
    pending_phase_advance_ticks: Option<u8>,

    /// シャッフル画面のスピナーアニメーション用 tick
    shuffle_spinner_ticks: u8,
}

impl App {
    /// 新しいアプリケーションを作成する
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Title,
            back_screen: CurrentScreen::Title,
            current_phase: GamePhase::Title,
            positions: BlockPosition::default(),
            should_quit: false,
            game: Game::new(),
            current: 0,
            turn: 1,

            text: AppText::new(),

            ticker_fps: TickerFps::new(),

            pending_phase_advance_ticks: None,
            shuffle_spinner_ticks: 0,
        }
    }

    /// ゲームを開始する
    pub fn start(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.back_screen = CurrentScreen::Main;
        self.text.disp = String::new();
        self.text.help = String::new();
        self.game.start();
    }

    /// ゲームをリセットする
    pub fn reset(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.back_screen = CurrentScreen::Main;
        self.text.disp = String::new();
        self.text.help = String::new();
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
                self.shuffle_spinner_ticks = 0;
                self.text.help = String::from("Shuffling the deck...");
            },
            GamePhase::Deal => {
                self.text.help = String::from("Dealing the cards...");
            },
            GamePhase::Playing => {
                self.text.help = String::from("Card strength: Ace > King > Queen > Jack > 10 > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2");
            },
            _ => {},
        }
    }

    /// 次のフェーズへの進行をスケジュールする
    pub fn schedule_phase_advance(&mut self, delay_ticks: u8) {
        if self.pending_phase_advance_ticks.is_none() {
            self.pending_phase_advance_ticks = Some(delay_ticks);
        }
    }

    /// チックを進める
    /// 次のフェーズへ進むタイミングなら `true` を返す
    pub fn tick(&mut self) -> bool {
        self.game.tick_shuffle();

        // 次のフェーズへの進行をスケジュールしていたら、その時間を進める
        let Some(ticks) = self.pending_phase_advance_ticks.as_mut() else {
            return false;
        };

        // 時間を進める
        *ticks = ticks.saturating_sub(1);

        // 時間が0になったら、次のフェーズへ進む
        if *ticks == 0 {
            self.pending_phase_advance_ticks = None;
            return true;
        }
        false
    }

    /// シャッフル画面のスピナー用フレーム（0..4）
    pub fn shuffle_spinner_frame(&self) -> usize {
        if self.current_screen != CurrentScreen::Shuffle {
            return 0;
        }

        self.shuffle_spinner_ticks as usize % 4
    }

    /// シャッフル画面のスピナーアニメーション用 tick を進める
    pub fn advance_shuffle_spinner(&mut self) {
        self.shuffle_spinner_ticks = self.shuffle_spinner_ticks.wrapping_add(1);
    }

    /// アプリケーションを終了する
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
