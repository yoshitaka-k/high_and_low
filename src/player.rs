use getset::{Getters, Setters};

use crate::constants::DEFAULT_CREDITS;

/// プレイヤーの情報を管理する構造体
#[derive(Getters, Setters)]
pub struct Player {
    #[getset(get = "pub", set = "pub")]
    pub name: String,

    #[getset(get = "pub", set = "pub")]
    pub credits: i32,

    #[getset(get = "pub", set = "pub")]
    pub win: u32,
    #[getset(get = "pub", set = "pub")]
    pub max_win: u32,
}

impl Player {
    /// 新しいプレイヤーを作成する
    pub fn new(name: String) -> Self {
        Self { name, credits: DEFAULT_CREDITS, win: 0, max_win: 0 }
    }

    /// ベットを行う
    pub fn credit_add(&mut self, bet: i32) {
        self.credits += bet;
    }
}
