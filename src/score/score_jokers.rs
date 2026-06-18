use crate::card::Edition;
use crate::joker::{Joker, JokerState};
use crate::levels::Hand;

pub fn score_jokers(
    jokers: &[JokerState],
    hand: Hand,
    mut base_chips: f32,
    mut base_mult: f32,
) -> [f32; 2] {
    let mut results = Vec::with_capacity(jokers.len());

    for joker in jokers {
        let mut chips = 0.0;
        let mut plus_mult = 0.0;
        let mut xmult = 0.0;

        // Apply Edition bonuses
        let edition = joker.edition();
        if edition == Edition::Foil as u8 {
            chips = 50.0;
        } else if edition == Edition::Holographic as u8 {
            plus_mult = 10.0;
        } else if edition == Edition::Polychrome as u8 {
            xmult = 1.5;
        }

        let id = joker.id() as usize;
        let score_fn = crate::score::jokers::SCORE_FNS[id];
        let [j_chips, j_plus_mult, j_xmult] = score_fn(joker, hand);

        chips += j_chips;
        plus_mult += j_plus_mult;
        if j_xmult != 0.0 {
            xmult = if xmult == 0.0 {
                j_xmult
            } else {
                xmult * j_xmult
            };
        }

        results.push([chips, plus_mult, xmult]);
    }
    for result in results {
        base_chips += result[0];
        base_mult += result[1];
        if result[2] != 0.0 {
            base_mult *= result[2];
        }
    }

    return [base_chips, base_mult];
}
