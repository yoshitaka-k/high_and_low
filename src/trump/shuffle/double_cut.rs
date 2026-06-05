use crate::trump::Card;
use crate::trump::shuffle::SimpleRand;

/// 山をだいたい三等分したうえで束の順番だけ入れ替える
pub fn double_cut(cards: &mut Vec<Card>) {
    let n = cards.len();
    if n < 3 {
        return;
    }

    let mut rng = SimpleRand::new();
    let jitter = (n / 10).max(1);

    let cut1 = ((n / 3) as isize + rng.next_range(-(jitter as isize)..=(jitter as isize)))
        .clamp(1, (n - 2) as isize) as usize;
    let cut2 = ((2 * n / 3) as isize + rng.next_range(-(jitter as isize)..=(jitter as isize)))
        .clamp((cut1 + 1) as isize, (n - 1) as isize) as usize;

    let mut rest = std::mem::take(cards);
    let pile_r = rest.split_off(cut2);
    let pile_m = rest.split_off(cut1);
    let pile_l = rest;

    let mut mixed = Vec::with_capacity(n);
    mixed.extend(pile_m);
    mixed.extend(pile_r);
    mixed.extend(pile_l);

    *cards = mixed;
}
