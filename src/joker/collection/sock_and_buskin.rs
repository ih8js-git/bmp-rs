use crate::card::Card;
use crate::card::core::Rank;
use crate::card::operations::get_card_rank;
use crate::joker::{Joker, JokerState};
use crate::levels::Hand;

pub fn score(
    _state: &JokerState,
    _hand: Hand,
    _chips: &mut f64,
    _mult: &mut f64,
) -> Result<(), &'static str> {
    Ok(())
}

pub fn retrigger(card: &Card, jokers: &[JokerState], _state: &JokerState) -> usize {
    let pareidolia = jokers.iter().any(|j| j.id() == Joker::Pareidolia as u8);
    let rank = get_card_rank(card);
    let is_face = pareidolia || matches!(rank, Rank::Jack | Rank::Queen | Rank::King);

    if is_face { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Suit, create_card};

    fn get_test_jokers(jokers: &[Joker]) -> Vec<JokerState> {
        jokers
            .iter()
            .map(|j| {
                let mut s = JokerState::new();
                s.set_id(*j as u8);
                s
            })
            .collect()
    }

    #[test]
    fn test_face_cards_retrigger() {
        let state = JokerState::new();

        let jack = create_card(Rank::Jack, Suit::Spades);
        let queen = create_card(Rank::Queen, Suit::Hearts);
        let king = create_card(Rank::King, Suit::Diamonds);

        assert_eq!(retrigger(&jack, &[], &state), 1);
        assert_eq!(retrigger(&queen, &[], &state), 1);
        assert_eq!(retrigger(&king, &[], &state), 1);
    }

    #[test]
    fn test_non_face_cards_do_not_retrigger() {
        let state = JokerState::new();

        let ace = create_card(Rank::Ace, Suit::Spades);
        let two = create_card(Rank::Two, Suit::Clubs);
        let ten = create_card(Rank::Ten, Suit::Diamonds);

        assert_eq!(retrigger(&ace, &[], &state), 0);
        assert_eq!(retrigger(&two, &[], &state), 0);
        assert_eq!(retrigger(&ten, &[], &state), 0);
    }

    #[test]
    fn test_pareidolia_makes_all_cards_retrigger() {
        let state = JokerState::new();
        let jokers = get_test_jokers(&[Joker::Pareidolia]);

        let two = create_card(Rank::Two, Suit::Clubs);
        let ace = create_card(Rank::Ace, Suit::Spades);
        let king = create_card(Rank::King, Suit::Diamonds);

        // Even non-face cards retrigger
        assert_eq!(retrigger(&two, &jokers, &state), 1);
        assert_eq!(retrigger(&ace, &jokers, &state), 1);
        // Face cards still only retrigger once
        assert_eq!(retrigger(&king, &jokers, &state), 1);
    }
}
