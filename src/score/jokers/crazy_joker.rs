use crate::joker::JokerState;
use crate::levels::Hand;

pub fn score(_state: &JokerState, hand: Hand) -> [f32; 3] {
    let mut mult = 0.0;

    let contains_straight = matches!(hand, Hand::StraightFlush | Hand::Straight);

    if contains_straight {
        mult += 12.0;
    }

    [0.0, mult, 0.0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Rank, Suit, create_card};
    use crate::decks::Deck;
    use crate::game::create_game_state;
    use crate::joker::Joker;
    use crate::score::core::get_score;

    #[test]
    fn test_crazy_joker_with_straight() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::CrazyJoker as u8);
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
        // Total chips: 50. Total mult: 4 + 12 (Crazy Joker) = 16.
        // Expected score: 50 * 16 = 800.0
        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 800.0);
    }

    #[test]
    fn test_crazy_joker_with_straight_flush() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::CrazyJoker as u8);
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
        // Total chips: 120. Total mult: 8 + 12 (Crazy Joker) = 20.
        // Expected score: 120 * 20 = 2400.0
        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 2400.0);
    }

    #[test]
    fn test_crazy_joker_without_straight() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::CrazyJoker as u8);
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
