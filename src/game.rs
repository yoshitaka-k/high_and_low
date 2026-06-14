use std::cmp::Ordering;

use getset::{Getters, MutGetters, Setters};

use crate::trump::{Card, Deck, shuffle::{ShufflePhase, ShuffleRunner}};

/// プレイヤーの選択
#[derive(PartialEq)]
pub enum PlayerChoice {
    High,
    Low,
}

/// ゲームの状態
#[derive(Default, Getters, MutGetters, Setters)]
pub struct Game {
    #[getset(get = "pub", get_mut = "pub")]
    deck: Deck,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    dealer_card: Option<Card>,
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    player_card: Option<Card>,

    #[getset(get = "pub", set = "pub")]
    choice: Option<PlayerChoice>,

    shuffle: Option<ShuffleRunner>,
    pending_phase_advance_ticks: Option<u8>,
}

impl Game {
    /// 新しいゲームを作成する
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),

            shuffle: None,
            pending_phase_advance_ticks: None,

            choice: None,

            dealer_card: None,
            player_card: None,
        }
    }

    /// 山札を用意して、シャッフルを開始する
    pub fn start(&mut self) {
        self.choice = None;
        self.dealer_card = None;
        self.player_card = None;

        self.deck = Deck::new();
        self.begin_shuffle();
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
    /// 次のフェーズへ進むタイミングなら `true` を返す
    pub fn tick(&mut self) -> bool {
        self.tick_shuffle();

        let Some(ticks) = self.pending_phase_advance_ticks.as_mut() else {
            return false;
        };
        *ticks = ticks.saturating_sub(1);
        if *ticks == 0 {
            self.pending_phase_advance_ticks = None;
            return true;
        }
        false
    }

    /// シャッフルを進める
    /// シャッフルが完了したら、山札を更新する
    fn tick_shuffle(&mut self) {
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

    /// 次のフェーズへの進行をスケジュールする
    pub fn schedule_phase_advance(&mut self, delay_ticks: u8) {
        if self.pending_phase_advance_ticks.is_none() {
            self.pending_phase_advance_ticks = Some(delay_ticks);
        }
    }

    /// 勝敗結果の表示ラベルを返す
    pub fn result_label(&self) -> &'static str {
        let (Some(dealer), Some(player), Some(choice)) =
            (self.dealer_card(), self.player_card(), self.choice())
        else {
            return "Draw!";
        };

        match player.rank_diff(dealer).cmp(&0) {
            Ordering::Equal => "Draw!",
            Ordering::Greater if *choice == PlayerChoice::High => "You win!",
            Ordering::Less if *choice == PlayerChoice::Low => "You win!",
            _ => "You lose!",
        }
    }
}
