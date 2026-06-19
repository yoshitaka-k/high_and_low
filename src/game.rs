use std::cmp::Ordering;

use getset::{Getters, MutGetters, Setters};

use crate::trump::{Card, Deck, shuffle::{ShufflePhase, ShuffleRunner}};

/// プレイヤーの選択
#[derive(PartialEq)]
pub enum PlayerChoice {
    High,
    Low,
}

/// 勝敗結果の表示ラベル
pub enum ResultLabel {
    Win,
    Lose,
    Draw,
}

/// ゲームの状態
#[derive(Default, Getters, MutGetters, Setters)]
pub struct Game {
    #[getset(get = "pub", get_mut = "pub")]
    deck: Deck,

    #[getset(get = "pub", set = "pub")]
    bet: i32,
    #[getset(get = "pub", set = "pub")]
    input_bet: i32,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    dealer_card: Option<Card>,
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    player_card: Option<Card>,

    #[getset(get = "pub", set = "pub")]
    choice: Option<PlayerChoice>,

    #[getset(get = "pub", set = "pub")]
    enter: bool,

    shuffle: Option<ShuffleRunner>,
}

impl Game {
    /// 新しいゲームを作成する
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),

            bet: 0,
            input_bet: 0,

            shuffle: None,

            choice: None,
            enter: false,

            dealer_card: None,
            player_card: None,
        }
    }

    /// 山札を用意して、シャッフルを開始する
    pub fn start(&mut self) {
        self.choice = None;
        self.enter = false;

        self.dealer_card = None;
        self.player_card = None;

        self.deck = Deck::new();
        self.begin_shuffle();
    }

    /// ゲームをリセットする
    pub fn reset(&mut self) {
        self.choice = None;
        self.enter = false;
        self.dealer_card = None;
        self.player_card = None;
    }

    /// シャッフル中かどうかを返す
    pub fn is_shuffling(&self) -> bool {
        self.shuffle
            .as_ref()
            .is_some_and(|s| s.is_active())
    }

    /// シャッフル中のフェーズ名（UI 表示用）
    pub fn shuffle_phase_label(&self) -> Option<&'static str> {
        self.shuffle
            .as_ref()
            .and_then(|s| s.phase())
            .map(ShufflePhase::label)
    }

    /// シャッフルを開始する
    /// シャッフルが既に開始されていたら、何もしない
    pub fn begin_shuffle(&mut self) {
        if self.is_shuffling() {
            return;
        }
        let cards = std::mem::take(self.deck.cards_mut());
        let runner = ShuffleRunner::new(cards);
        self.deck.replace_cards(runner.cards().to_vec());
        self.shuffle = Some(runner);
    }

    /// シャッフルを進める
    /// シャッフルが完了したら、山札を更新する
    pub fn tick_shuffle(&mut self) {
        let Some(runner) = &mut self.shuffle else {
            if self.deck.is_empty() {
                self.begin_shuffle();
            }
            return;
        };

        if let Some(cards) = runner.tick() {
            self.deck.replace_cards(cards);
            self.shuffle = None;
            return;
        }

        self.deck.replace_cards(runner.cards().to_vec());
    }

    /// プレイヤーのカードとディーラーのカードのランクの大小関係を返す
    pub fn player_card_diff(&self) -> ResultLabel {
        let (Some(dealer), Some(player), Some(choice)) =
            (self.dealer_card(), self.player_card(), self.choice())
        else {
            return ResultLabel::Draw;
        };
        match player.rank_diff(dealer).cmp(&0) {
            Ordering::Equal => ResultLabel::Draw,
            Ordering::Greater => {
                if *choice == PlayerChoice::High {
                    ResultLabel::Win
                } else {
                    ResultLabel::Lose
                }
            }
            Ordering::Less => {
                if *choice == PlayerChoice::Low {
                    ResultLabel::Win
                } else {
                    ResultLabel::Lose
                }
            }
        }
    }

    /// 勝敗結果の表示ラベルを返す
    pub fn result_label(&self) -> &'static str {
        match self.player_card_diff() {
            ResultLabel::Win => "You win!",
            ResultLabel::Lose => "You lose!",
            ResultLabel::Draw => "Draw!",
        }
    }
}
