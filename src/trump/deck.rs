use getset::{Getters, MutGetters, Setters};
use crate::trump::{
    Card,
    card_enum::{rank::Rank, suit::Suit}
};

const SUITS: [Suit; 4] = [
    Suit::Heart,
    Suit::Diamond,
    Suit::Clover,
    Suit::Spade,
];
const RANKS: [Rank; 13] = [
    Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
    Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
    Rank::Jack, Rank::Queen, Rank::King
];

/// カードの山札を管理する構造体
#[derive(Default, Getters, MutGetters, Setters)]
pub struct Deck {
    #[getset(get = "pub", get_mut = "pub")]
    cards: Vec<Card>,
}

impl Deck {
    /// 新しい山札を作成する
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in SUITS {
            for rank in RANKS {
                cards.push(Card::new(suit, rank));
            }
        }

        Self { cards: cards }
    }

    /// 山札の長さを取得する
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// 山札が空かどうかを取得する
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// 山札からカードを引く
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// 山札にカードを追加する
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// 山札を置き換える
    pub fn replace_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards;
    }
}
