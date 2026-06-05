use getset::{Getters, MutGetters, Setters};
use crate::game::Game;

#[derive(Default, Getters, MutGetters, Setters)]
pub struct App {
    #[getset(get = "pub", get_mut = "pub")]
    game: Game,
    pub should_quit: bool,
    pub text: String,
}

impl App {
    /// 新しいアプリケーションを作成する
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            should_quit: false,
            text: String::new(),
        }
    }

    /// ゲームを開始する
    pub fn start(&mut self) {
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
