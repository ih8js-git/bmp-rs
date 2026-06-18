use crate::card::core::{Card, Edition, Enhancement};
use crate::card::operations::{get_card_edition, get_card_enhancement};
use crate::joker::JokerState;
use crate::levels::{Hand, hand_base_chips_and_mult};

pub fn score_played_cards(
    cards: &[Card],
    scoring_indices: Vec<usize>,
    hand: Hand,
    level: u16,
    jokers: &[JokerState],
) -> [f32; 2] {
    let mut results = Vec::with_capacity(cards.len());

    let (base_chips_u16, base_mult_u16) = hand_base_chips_and_mult(level, hand);
    let mut base_chips = base_chips_u16 as f32;
    let mut base_mult = base_mult_u16 as f32;

    for &idx in &scoring_indices {
        let card = &cards[idx];
        let mut chips = card.chips as f32;
        let mut plus_mult = 0.0;
        let mut xmult = 0.0;

        // Apply Edition bonuses
        let edition = get_card_edition(card);
        if edition == Edition::Foil {
            chips += 50.0;
        } else if edition == Edition::Holographic {
            plus_mult += 10.0;
        } else if edition == Edition::Polychrome {
            xmult = 1.5;
        }

        // Apply Enhancement bonuses
        // We aren't worrying about RNG (Lucky cards) or Jokers right now
        let enhancement = get_card_enhancement(card);
        match enhancement {
            Enhancement::Bonus => chips += 30.0,
            Enhancement::Mult => plus_mult += 4.0,
            Enhancement::Glass => {
                xmult = if xmult == 0.0 { 2.0 } else { xmult * 2.0 };
            }
            Enhancement::Stone => chips += 50.0,
            // Steel only scores when held in hand, not when played
            // Lucky requires RNG, which we are ignoring for now
            _ => {}
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
