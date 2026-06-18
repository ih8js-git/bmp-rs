use crate::joker::JokerState;
use crate::levels::Hand;

pub fn score(_state: &JokerState, hand: Hand) -> [f32; 3] {
    let mut mult = 0.0;

    // Jolly Joker: +8 Mult if played hand contains a Pair
    // TODO: This doesn't cover all cases because a flush might have a pair in it.
    let contains_pair = match hand {
        Hand::Pair
        | Hand::TwoPair
        | Hand::ThreeOfAKind
        | Hand::FullHouse
        | Hand::FourOfAKind
        | Hand::FiveOfAKind
        | Hand::FlushHouse
        | Hand::FlushFive => true,
        _ => false,
    };

    if contains_pair {
        mult += 8.0;
    }

    [0.0, mult, 0.0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Rank, Suit, create_test_card};
    use crate::decks::{Deck, create_game_state};
    use crate::joker::Joker;
    use crate::score::core::get_score;

    #[test]
    fn test_jolly_joker_with_pair() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::JollyJoker as u8);
        state.jokers.push(joker_state);

        let mut hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
            create_test_card(Rank::Two, Suit::Clubs),
            create_test_card(Rank::Three, Suit::Diamonds),
            create_test_card(Rank::Four, Suit::Spades),
        ];

        // We expect Pair (10 chips, 2 mult) + played cards (10 + 10 = 20 chips)
        // Total chips: 30. Total mult: 2 + 8 (from Joker) = 10.
        // Expected score: 300.0
        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 300.0);
    }

    #[test]
    fn test_jolly_joker_without_pair() {
        let mut state = create_game_state(Deck::Red);

        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::JollyJoker as u8);
        state.jokers.push(joker_state);

        let mut hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::Queen, Suit::Hearts),
            create_test_card(Rank::Two, Suit::Clubs),
            create_test_card(Rank::Three, Suit::Diamonds),
            create_test_card(Rank::Four, Suit::Spades),
        ];

        // We expect High Card base at level 1: (5 chips, 1 mult). + played cards (10 = 10)
        // Total chips: 15. Total mult: 1.
        // Expected score: 15 * 1 = 15.0
        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 15.0);
    }
}
