use crate::trump::Card;

use super::deal::{deal_shuffle, DealParams};
use super::double_cut::double_cut;
use super::hindu::{hindu_shuffle, HinduParams};
use super::riffle::{riffle_shuffle, RiffleParams};

/// フェーズ間の待機 tick 数（200ms × N ≒ 見える間隔）
pub const SHUFFLE_PHASE_PAUSE_TICKS: u8 = 3;

/// 今どのシャッフル手法を実行中か
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShufflePhase {
    Hindu,
    Riffle,
    Deal,
    DoubleCut,
}

impl ShufflePhase {
    /// フェーズ名を返す
    pub fn label(self) -> &'static str {
        match self {
            Self::Hindu => "Hindu",
            Self::Riffle => "Riffle",
            Self::Deal => "Deal",
            Self::DoubleCut => "Double cut",
        }
    }

    /// 次のフェーズを返す
    fn next(self) -> Option<Self> {
        match self {
            Self::Hindu => Some(Self::Riffle),
            Self::Riffle => Some(Self::Deal),
            Self::Deal => Some(Self::DoubleCut),
            Self::DoubleCut => None,
        }
    }
}

/// Tick ごとに 1 フェーズずつ進める。ロジック本体は各 `*_shuffle` 関数。
pub struct ShuffleRunner {
    cards: Vec<Card>,
    phase: Option<ShufflePhase>,
    wait_ticks: u8,
}

impl ShuffleRunner {
    /// 新しいシャッフルランナーを作成する
    pub fn new(cards: Vec<Card>) -> Self {
        Self {
            cards,
            phase: Some(ShufflePhase::Hindu),
            wait_ticks: 0,
        }
    }

    /// シャッフルがアクティブかどうかを返す
    pub fn is_active(&self) -> bool {
        self.phase.is_some()
    }

    /// 現在のフェーズを返す
    pub fn phase(&self) -> Option<ShufflePhase> {
        self.phase
    }

    /// 山札を返す
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// 1 tick 進める。全フェーズ完了時に Some(cards) を返す。
    pub fn tick(&mut self) -> Option<Vec<Card>> {
        if self.phase.is_none() {
            return None;
        }

        if self.wait_ticks > 0 {
            self.wait_ticks -= 1;
            return None;
        }

        let phase = self.phase.take().expect("phase checked above");
        match phase {
            ShufflePhase::Hindu => hindu_shuffle(&mut self.cards, &HinduParams::default()),
            ShufflePhase::Riffle => riffle_shuffle(&mut self.cards, &RiffleParams::default()),
            ShufflePhase::Deal => deal_shuffle(&mut self.cards, &DealParams::default()),
            ShufflePhase::DoubleCut => double_cut(&mut self.cards),
        }

        match phase.next() {
            Some(next) => {
                self.phase = Some(next);
                self.wait_ticks = SHUFFLE_PHASE_PAUSE_TICKS;
                None
            }
            None => Some(std::mem::take(&mut self.cards)),
        }
    }
}
