use crate::trump::constants::{
    JOKER_STR_RANK,
    JOKER_TO_RANK,
};

use crate::trump::card_enum::suit::Suit;
use crate::trump::card_enum::rank::Rank;

/// カードの情報
#[derive(Debug, Clone)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    /// 新しいカードを作成する
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    /// カードのスートを取得する
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// カードのランクを取得する
    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// カードが等しいかどうかを取得する
    pub fn equals(&self, other: &Card) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }

    /// カードがジョーカーかどうかを取得する
    pub fn is_joker(&self) -> bool {
        self.suit == Suit::Joker
    }

    /// カードがエースかどうかを取得する
    pub fn is_ace_card(&self) -> bool {
        self.rank == Rank::Ace
    }

    /// カードが絵札かどうかを取得する
    pub fn is_face_card(&self) -> bool {
        self.rank == Rank::Jack
        || self.rank == Rank::Queen
        || self.rank == Rank::King
    }

    /// 手札表示用の並び: スート（h → d → c → s → j）、同スート内はランクの数値順。
    pub fn sort_tuple(&self) -> (u8, u16) {
        let suit = match self.suit {
            Suit::Heart => 0,
            Suit::Diamond => 1,
            Suit::Clover => 2,
            Suit::Spade => 3,
            Suit::Joker => 4,
        };
        let rank = self.rank.as_usize() as u16;
        (suit, rank)
    }

    /// カードのランクを計算する
    pub fn calc_rank(&self) -> usize {
        match self.suit {
            Suit::Joker => {
                JOKER_TO_RANK
            }
            _ => {
                self.rank.calc_rank()
            }
        }
    }

    /// カードのランクの差を計算する
    pub fn rank_diff(&self, other: &Card) -> isize {
        self.calc_rank() as isize - other.calc_rank() as isize
    }

    /// カードのスートを表示する
    pub fn disp_suit(&self) -> &'static str {
        self.suit.icon()
    }

    /// カードのランクを表示する
    pub fn disp_rank(&self) -> &'static str {
        match self.suit {
            Suit::Joker => {
                JOKER_STR_RANK
            }
            _ => {
                self.rank.disp_rank()
            }
        }
    }

    /// カードの名前を取得する
    pub fn name(&self) -> String {
        match self.disp_rank() {
            "" => format!("{}{}", self.disp_suit(), self.rank.disp_rank()),
            rank => format!("{}{}", self.disp_suit(), rank),
        }
    }
}

impl std::fmt::Display for Card {
    /// スート・ランク表示
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.disp_rank() {
            "" => write!(f, "{}{}", self.disp_suit(), self.rank.disp_rank()),
            rank => write!(f, "{}{}", self.disp_suit(), rank),
        }
    }
}
