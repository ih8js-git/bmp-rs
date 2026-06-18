use crate::joker::JokerState;
use crate::levels::Hand;

pub fn score(_state: &JokerState, _hand: Hand) -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

use crate::card::Card;
use crate::card::core::{Enhancement, Suit};
use crate::card::operations::{get_card_enhancement, get_card_suit};

pub fn card_score(_state: &JokerState, card: &Card) -> [f32; 3] {
    if get_card_suit(card) == Suit::Diamonds || get_card_enhancement(card) == Enhancement::Wild {
        [0.0, 3.0, 0.0]
    } else {
        [0.0, 0.0, 0.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Rank, create_card};

    #[test]
    fn test_greedy_joker() {
        let joker_state = JokerState::new();

        // Test Diamond card
        let diamond_card = create_card(Rank::Two, Suit::Diamonds);
        assert_eq!(card_score(&joker_state, &diamond_card), [0.0, 3.0, 0.0]);

        // Test non-Diamond card
        let spade_card = create_card(Rank::Two, Suit::Spades);
        assert_eq!(card_score(&joker_state, &spade_card), [0.0, 0.0, 0.0]);
    }
}
