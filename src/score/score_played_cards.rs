use crate::card::core::{Card, Edition, Enhancement, Seal};
use crate::card::operations::get_card_seal;
use crate::card::{get_card_edition, get_card_enhancement};
use crate::joker::JokerState;
use crate::joker::fn_arrays::card_score::CARD_SCORE_FNS;
use crate::levels::{Hand, hand_base_chips_and_mult};
use crate::score::core::count_retrigger_jokers;

pub fn score_played_cards(
    cards: &[Card],
    scoring_indices: Vec<usize>,
    hand: Hand,
    level: u16,
    jokers: &[JokerState],
) -> [f64; 2] {
    let (base_chips_u16, base_mult_u16) = hand_base_chips_and_mult(level, hand);
    let mut chips = base_chips_u16 as f64;
    let mut mult = base_mult_u16 as f64;

    for &idx in &scoring_indices {
        let card = &cards[idx];
        let mut trigger_count = 1;
        if get_card_seal(card) == Seal::Red {
            trigger_count += 1;
        }
        let trigger_count = trigger_count + count_retrigger_jokers(card, jokers);
        let card_enhancement = get_card_enhancement(card);
        let card_edition = get_card_edition(card);

        for t in 0..trigger_count {
            chips += card.chips as f64;
            match card_enhancement {
                Enhancement::None => (),
                Enhancement::Bonus => chips += 30.0,
                Enhancement::Mult => mult += 4.0,
                Enhancement::Wild => (),
                Enhancement::Glass => mult *= 2.0,
                Enhancement::Steel => (),
                Enhancement::Stone => chips += 50.0,
                Enhancement::Gold => (),
                // TODO: Add RNG logic
                Enhancement::Lucky => (),
            }
            match card_edition {
                Edition::None => (),
                Edition::Foil => chips += 50.0,
                Edition::Holographic => mult += 10.0,
                Edition::Polychrome => mult *= 1.5,
                Edition::Negative => {
                    panic!("Negative edition playing card. How did we even manage to get here?")
                }
            }

            for joker in jokers {
                let id = joker.id() as usize;
                let def = crate::joker::core::JOKER_DEFS[id];
                if def.trigger_time() == crate::joker::core::TriggerTime::CardScored {
                    let card_score_fn = CARD_SCORE_FNS[id];
                    card_score_fn(joker, card, &mut chips, &mut mult).unwrap();
                }
            }
        }
    }

    [chips, mult]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Rank, Suit, create_card};
    use crate::card::operations::{set_card_edition, set_card_enhancement, set_card_seal};
    use crate::decks::Deck;
    use crate::game::create_game_state;
    use crate::score::core::get_score;

    fn setup_card(enhancement: Enhancement, edition: Edition, seal: Seal) -> Card {
        let mut card = create_card(Rank::Two, Suit::Spades);
        set_card_enhancement(&mut card, enhancement);
        set_card_edition(&mut card, edition);
        set_card_seal(&mut card, seal);
        card
    }

    fn score_card(card: Card) -> f64 {
        let mut state = create_game_state(Deck::Red);
        let mut hand = vec![card];
        get_score(&mut state, &mut hand)
    }

    #[test]
    fn test_enhancements() {
        let cases = [
            (Enhancement::None, 7.0),   // 5 (High Card base) + 2 = 7 chips, 1 mult -> 7
            (Enhancement::Bonus, 37.0), // 7 + 30 = 37 chips, 1 mult -> 37
            (Enhancement::Mult, 35.0),  // 7 chips, 1 + 4 = 5 mult -> 35
            (Enhancement::Glass, 14.0), // 7 chips, 1 * 2 = 2 mult -> 14
            (Enhancement::Stone, 57.0), // 7 + 50 = 57 chips, 1 mult -> 57
            (Enhancement::Wild, 7.0),
            (Enhancement::Steel, 7.0),
            (Enhancement::Gold, 7.0),
            (Enhancement::Lucky, 7.0),
        ];

        for (enhancement, expected) in cases {
            let card = setup_card(enhancement, Edition::None, Seal::None);
            let score = score_card(card);
            assert_eq!(score, expected, "Failed for enhancement {:?}", enhancement);
        }
    }

    #[test]
    fn test_editions() {
        let cases = [
            (Edition::None, 7.0),
            (Edition::Foil, 57.0),        // 7 + 50 = 57 chips, 1 mult -> 57
            (Edition::Holographic, 77.0), // 7 chips, 1 + 10 = 11 mult -> 77
            (Edition::Polychrome, 10.5),  // 7 chips, 1 * 1.5 = 1.5 mult -> 10.5
        ];

        for (edition, expected) in cases {
            let card = setup_card(Enhancement::None, edition, Seal::None);
            let score = score_card(card);
            assert_eq!(score, expected, "Failed for edition {:?}", edition);
        }
    }

    #[test]
    #[should_panic(expected = "Negative edition playing card")]
    fn test_negative_edition_panics() {
        let card = setup_card(Enhancement::None, Edition::Negative, Seal::None);
        score_card(card);
    }

    #[test]
    fn test_red_seal() {
        // Red seal triggers the card one extra time
        let card = setup_card(Enhancement::None, Edition::None, Seal::Red);
        assert_eq!(score_card(card), 9.0); // 5 + 2 + 2 = 9 chips, 1 mult -> 9

        // Red seal with Foil edition (+50 chips per trigger)
        let foil_red = setup_card(Enhancement::None, Edition::Foil, Seal::Red);
        assert_eq!(score_card(foil_red), 109.0); // 5 + (2+50) + (2+50) = 109 chips, 1 mult -> 109
    }
}
