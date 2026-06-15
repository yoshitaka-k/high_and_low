use crate::trump::constants::{
    ACE_STR_RANK,
    ACE_TO_RANK,

    JACK_STR_RANK,
    JACK_TO_RANK,

    QUEEN_STR_RANK,
    QUEEN_TO_RANK,

    KING_STR_RANK,
    KING_TO_RANK,
};

/// カードのランクを管理する列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    /// ランクを数値に変換する
    pub fn as_usize(self) -> usize {
        self as usize
    }

    /// ランクをisizeに変換する
    pub fn as_isize(self) -> isize {
        self as isize
    }

    /// ランクを表示する
    pub fn disp_rank(self) -> &'static str {
        match self {
            Self::Ace => ACE_STR_RANK,
            Self::Jack => JACK_STR_RANK,
            Self::Queen => QUEEN_STR_RANK,
            Self::King => KING_STR_RANK,
            _ => "",
        }
    }

    /// ランクを計算用の値にする
    pub fn calc_rank(self) -> usize {
        match self {
            Self::Ace => ACE_TO_RANK as usize,
            Self::Jack => JACK_TO_RANK as usize,
            Self::Queen => QUEEN_TO_RANK as usize,
            Self::King => KING_TO_RANK as usize,
            _ => self as usize,
        }
    }
}
