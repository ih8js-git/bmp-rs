use crate::joker::JokerState;
use crate::levels::Hand;

pub fn score(
    _state: &JokerState,
    _hand: Hand,
    _chips: &mut f64,
    mult: &mut f64,
) -> Result<(), &'static str> {
    *mult += 4.0;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Rank, Suit, create_card};
    use crate::decks::Deck;
    use crate::game::state::create_game_state;
    use crate::joker::Joker;
    use crate::score::core::get_score;

    #[test]
    fn test_joker_score_integration() {
        let mut state = create_game_state(Deck::Red);

        // A single Two of Spades makes a High Card hand
        let mut hand = vec![create_card(Rank::Two, Suit::Spades)];

        // Score without the joker
        let score_without = get_score(&mut state, &mut hand);

        // High Card base: 5 chips, 1 mult. Two of Spades gives +2 chips.
        // Chips: 7, Mult: 1 -> Score: 7.0
        assert_eq!(score_without, 7.0, "Score without Joker should be 7.0");

        // Add the basic Joker
        let mut joker = JokerState::new();
        joker.set_id(Joker::Joker as u8);
        state.jokers.push(joker);

        // Score with the joker
        let score_with = get_score(&mut state, &mut hand);

        // High Card base: 5 chips, 1 mult. Two of Spades gives +2 chips.
        // Joker gives +4 mult.
        // Chips: 7, Mult: 5 -> Score: 35.0
        assert_eq!(score_with, 35.0, "Score with Joker should be 35.0");
    }
}
