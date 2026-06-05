use crate::trump::shuffle::center_position;
use crate::trump::Card;
use crate::trump::shuffle::SimpleRand;

/// リフル回数・1回あたりに落とす枚数の上限の指定
pub struct RiffleParams {
    /// リフルを繰り返す回数
    pub iterations: usize,
    /// 山から一度に落とす枚数の上限（1〜この値の乱数）
    pub max_chunk: usize,
}

impl Default for RiffleParams {
    fn default() -> Self {
        Self {
            iterations: 1,
            max_chunk: 3,
        }
    }
}

/// 山札の切り方（リフル式）
pub fn riffle_shuffle(cards: &mut Vec<Card>, params: &RiffleParams) {
    let iterations = params.iterations.max(1);
    let max_chunk = params.max_chunk.max(1);

    for _ in 0..iterations {
        riffle_shuffle_once(cards, max_chunk);
    }
}

fn riffle_shuffle_once(cards: &mut Vec<Card>, max_chunk: usize) {
    if cards.len() < 2 {
        return;
    }

    let mut rng = SimpleRand::new();
    let cut = center_position(cards.len());

    let mut right = cards.split_off(cut);
    let mut left = std::mem::take(cards);
    let mut mixed = Vec::with_capacity(left.len() + right.len());

    while !left.is_empty() || !right.is_empty() {
        let take_left = if left.is_empty() {
            false
        } else if right.is_empty() {
            true
        } else {
            let total = left.len() + right.len();
            let r = rng.next_range(0..=(total as isize - 1)) as usize;
            r < left.len()
        };

        let pile = if take_left { &mut left } else { &mut right };
        let chunk_cap = pile.len().min(max_chunk);
        let take_n = rng.next_range(1..=chunk_cap as isize) as usize;
        let start = pile.len() - take_n;
        mixed.extend(pile.drain(start..));
    }

    *cards = mixed;
}
