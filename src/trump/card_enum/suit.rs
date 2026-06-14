use crate::trump::constants::{
    SUIT_STR_HEART,
    SUIT_STR_DIAMOND,
    SUIT_STR_CLOVER,
    SUIT_STR_SPADE,
    SUIT_STR_JOKER,

    SUIT_ICON_HEART,
    SUIT_ICON_DIAMOND,
    SUIT_ICON_CLOVER,
    SUIT_ICON_SPADE,
    SUIT_ICON_JOKER,
};

/// カードのスートを管理する列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamond,
    Clover,
    Spade,
    Joker,
}

impl Suit {
    /// 文字列からスートを取得する
    pub fn from_str(suit: &str) -> Self {
        match suit {
            SUIT_STR_HEART => Self::Heart,
            SUIT_STR_DIAMOND => Self::Diamond,
            SUIT_STR_CLOVER => Self::Clover,
            SUIT_STR_SPADE => Self::Spade,
            SUIT_STR_JOKER => Self::Joker,
            _ => Self::Joker,
        }
    }

    /// スートを文字列に変換する
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Heart => SUIT_STR_HEART,
            Self::Diamond => SUIT_STR_DIAMOND,
            Self::Clover => SUIT_STR_CLOVER,
            Self::Spade => SUIT_STR_SPADE,
            Self::Joker => SUIT_STR_JOKER,
        }
    }

    /// スートのアイコンを取得する
    pub fn icon(self) -> &'static str {
        match self {
            Self::Heart => SUIT_ICON_HEART,
            Self::Diamond => SUIT_ICON_DIAMOND,
            Self::Clover => SUIT_ICON_CLOVER,
            Self::Spade => SUIT_ICON_SPADE,
            Self::Joker => SUIT_ICON_JOKER,
        }
    }
}
