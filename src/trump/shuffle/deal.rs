use crate::trump::Card;
use crate::trump::shuffle::SimpleRand;

/// ディール回数・山数の指定
pub struct DealParams {
    /// ディールを繰り返す回数
    pub iterations: usize,
    /// 何この山を作るか指定
    pub pile_count: usize,
}

impl Default for DealParams {
    fn default() -> Self {
        Self {
            iterations: 1,
            pile_count: 5,
        }
    }
}

/// 山札の切り方（ディール方式）
pub fn deal_shuffle(cards: &mut Vec<Card>, params: &DealParams) {
    let iterations = params.iterations.max(1);
    let pile_count = params.pile_count.max(1);

    for _ in 0..iterations {
        deal_shuffle_once(cards, pile_count);
    }
}

fn deal_shuffle_once(cards: &mut Vec<Card>, pile_count: usize) {
    if cards.len() < 2 {
        return;
    }

    let total_cards = cards.len();
    let mut left = std::mem::take(cards);
    let mut piles: Vec<Vec<Card>> = (0..pile_count).map(|_| Vec::new()).collect();

    for (i, card) in left.drain(..).rev().enumerate() {
        piles[i % pile_count].push(card);
    }

    let mut rng = SimpleRand::new();
    let mut mixed = Vec::with_capacity(total_cards);
    for pile in &mut piles {
        let take_n = rng.next_range(0..=pile.len() as isize) as usize;
        let start = pile.len() - take_n;
        mixed.extend(pile.drain(start..));
        mixed.extend(pile.drain(..));
    }

    *cards = mixed;
}
