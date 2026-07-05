use crate::joker::JokerState;
use crate::levels::Hand;

pub fn score(
    _state: &JokerState,
    hand: Hand,
    chips: &mut f64,
    _mult: &mut f64,
) -> Result<(), &'static str> {
    let contains_straight = matches!(hand, Hand::StraightFlush | Hand::Straight);

    if contains_straight {
        *chips += 100.0;
    }

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
    fn test_devious_joker_with_straight() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::DeviousJoker as u8);
        state.jokers.push(joker_state);

        let mut hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Three, Suit::Hearts),
            create_card(Rank::Four, Suit::Clubs),
            create_card(Rank::Five, Suit::Diamonds),
            create_card(Rank::Six, Suit::Spades),
        ];

        // Straight: 30 chips, 4 mult.
        // Played cards: 2 + 3 + 4 + 5 + 6 = 20 chips.
        // Total chips: 50 + 100 (Devious Joker) = 150 chips.
        // Total mult: 4.
        // Expected score: 150 * 4 = 600.0
        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 600.0);
    }

    #[test]
    fn test_devious_joker_with_straight_flush() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::DeviousJoker as u8);
        state.jokers.push(joker_state);

        let mut hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Three, Suit::Spades),
            create_card(Rank::Four, Suit::Spades),
            create_card(Rank::Five, Suit::Spades),
            create_card(Rank::Six, Suit::Spades),
        ];

        // Straight Flush: 100 chips, 8 mult.
        // Played cards: 2 + 3 + 4 + 5 + 6 = 20 chips.
        // Total chips: 120 + 100 (Devious Joker) = 220 chips.
        // Total mult: 8.
        // Expected score: 220 * 8 = 1760.0
        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 1760.0);
    }

    #[test]
    fn test_devious_joker_without_straight() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::DeviousJoker as u8);
        state.jokers.push(joker_state);

        let mut hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::Queen, Suit::Hearts),
            create_card(Rank::Two, Suit::Clubs),
            create_card(Rank::Three, Suit::Diamonds),
            create_card(Rank::Four, Suit::Spades),
        ];

        // High Card: 5 chips, 1 mult.
        // Played cards (only highest scores): King = 10 chips.
        // Total chips: 15. Total mult: 1.
        // Expected score: 15 * 1 = 15.0
        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 15.0);
    }
}
