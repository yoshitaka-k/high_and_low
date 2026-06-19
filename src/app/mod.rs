mod phase_enter;
mod text;
mod color;

use crate::components::TickerFps;
use crate::app::{text::AppText, color::AppColor};
use crate::game::Game;
use crate::player::Player;
use crate::rendar::block_position::BlockPosition;
use crate::constants::DEFAULT_CREDITS;

/// 現在の画面を表す列挙型
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CurrentScreen {
    Title,
    Main,
    End,
    BetInput,
    Shuffle,
    Exiting,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GamePhase {
    Title,
    BetInput,
    Setup,
    Shuffle,
    Deal,
    Playing,
    Result,
    End,
}

/// アプリケーションの状態を表す構造体
pub struct App {
    pub current_screen: CurrentScreen,
    pub back_screen: CurrentScreen,
    pub current_phase: GamePhase,

    pub positions: BlockPosition,

    pub should_quit: bool,

    pub game: Game,
    pub player: Player,

    pub current: usize,
    pub turn: usize,

    pub text: AppText,
    pub color: AppColor,

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
            player: Player::new("Player".to_string(), DEFAULT_CREDITS),
            current: 0,
            turn: 1,

            text: AppText::default(),
            color: AppColor::default(),

            ticker_fps: TickerFps::new(),

            pending_phase_advance_ticks: None,
            shuffle_spinner_ticks: 0,
        }
    }

    /// ゲームを開始する
    pub fn start(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.back_screen = CurrentScreen::Main;
        self.text.choice = String::new();
        self.text.help = String::new();
        self.game.start();
    }

    /// ゲームをリセットする
    pub fn reset(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.back_screen = CurrentScreen::Main;
        self.text.choice = String::new();
        self.text.help = String::new();
        self.game.reset();
    }

    /// 次のフェーズへ進む
    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            GamePhase::Title => GamePhase::BetInput,
            GamePhase::BetInput => GamePhase::Setup,
            GamePhase::Setup => GamePhase::Shuffle,
            GamePhase::Shuffle => GamePhase::Deal,
            GamePhase::Deal => GamePhase::Playing,
            GamePhase::Playing => GamePhase::Result,
            GamePhase::Result => GamePhase::End,
            GamePhase::End => {
                //ゲームオーバーになったからタイトルへ戻る
                if *self.player.credits() <= 0 {
                    GamePhase::Title

                } else {
                    self.turn += 1;

                    if self.game.deck().len() < 2 {
                        GamePhase::Setup
                    } else {
                        self.reset();
                        GamePhase::Deal
                    }

                }
            }
        };
        phase_enter::dispatch(self);
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
