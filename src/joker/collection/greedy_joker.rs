use crate::joker::JokerState;

use crate::card::Card;
use crate::card::core::{Enhancement, Suit};
use crate::card::operations::{get_card_enhancement, get_card_suit};
use crate::levels::Hand;

pub fn score(
    _state: &JokerState,
    _hand: Hand,
    _chips: &mut f64,
    _mult: &mut f64,
) -> Result<(), &'static str> {
    Ok(())
}

pub fn card_score(
    _state: &JokerState,
    card: &Card,
    chips: &mut f64,
    mult: &mut f64,
) -> Result<(), &'static str> {
    if get_card_suit(card) == Suit::Diamonds || get_card_enhancement(card) == Enhancement::Wild {
        *mult += 3.0;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Rank, create_card};
    use crate::decks::Deck;
    use crate::game::create_game_state;
    use crate::joker::Joker;
    use crate::score::core::get_score;

    #[test]
    fn test_greedy_joker() {
        let mut state = create_game_state(Deck::Red);
        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::GreedyJoker as u8);
        state.jokers.push(joker_state);

        // Test Diamond card
        let diamond_card = create_card(Rank::Two, Suit::Diamonds);
        let mut hand_diamond = vec![diamond_card];

        // Base chips: 5 (High card) + 2 (Two) = 7.
        // Base mult: 1 (High card) + 3 (Greedy Joker) = 4.
        // Score: 7 * 4 = 28.
        assert_eq!(get_score(&mut state, &mut hand_diamond), 28.0);

        // Test non-Diamond card
        let spade_card = create_card(Rank::Two, Suit::Spades);
        let mut hand_spade = vec![spade_card];

        // Base chips: 5 (High card) + 2 (Two) = 7.
        // Base mult: 1 (High card) + 0 = 1.
        // Score: 7 * 1 = 7.
        assert_eq!(get_score(&mut state, &mut hand_spade), 7.0);
    }
}
