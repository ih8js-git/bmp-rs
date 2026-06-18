use crate::card::core::create_card;
use crate::card::{Card, Rank, Suit};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Deck {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Magic,
    Nebula,
    Ghost,
    Abandoned,
    Checkered,
    Zodiac,
    Painted,
    Anaglyph,
    Plasma,
    Erratic,
}

pub fn create_default_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            cards.push(create_card(rank, suit));
        }
    }
    return cards;
}

pub fn create_abandoned_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            if rank != Rank::Jack && rank != Rank::Queen && rank != Rank::King {
                cards.push(create_card(rank, suit));
            }
        }
    }
    return cards;
}

pub fn create_checkered_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            let mut current_suit = suit;
            if current_suit == Suit::Clubs {
                current_suit = Suit::Spades;
            } else if current_suit == Suit::Diamonds {
                current_suit = Suit::Hearts;
            }
            cards.push(create_card(rank, current_suit));
        }
    }
    return cards;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Voucher;
    use crate::consumable::{Consumable, Spectral, Tarot};
    use crate::game::create_game_state;
    use crate::has_voucher;

    #[test]
    fn test_base_deck_properties() {
        // Erratic deck falls back to the base case
        let state = create_game_state(Deck::Plasma);
        assert_eq!(state.deck.len(), 52);
        assert_eq!(state.discards, 3);
        assert_eq!(state.hands, 4);
        assert_eq!(state.balance, 4);
        assert_eq!(state.joker_slots, 5);
        assert_eq!(state.consumable_slots, 2);
        assert_eq!(state.hand_size, 8);
        assert_eq!(state.vouchers, 0);
        assert!(state.consumables.is_empty());
    }

    #[test]
    fn test_red_deck() {
        let state = create_game_state(Deck::Red);
        assert_eq!(state.discards, 4); // base 3 + 1
        assert_eq!(state.hands, 4);
    }

    #[test]
    fn test_blue_deck() {
        let state = create_game_state(Deck::Blue);
        assert_eq!(state.hands, 5); // base 4 + 1
        assert_eq!(state.discards, 3);
    }

    #[test]
    fn test_yellow_deck() {
        let state = create_game_state(Deck::Yellow);
        assert_eq!(state.balance, 14); // base 4 + 10
    }

    #[test]
    fn test_black_deck() {
        let state = create_game_state(Deck::Black);
        assert_eq!(state.joker_slots, 6); // base 5 + 1
        assert_eq!(state.hands, 3); // base 4 - 1
    }

    #[test]
    fn test_magic_deck() {
        let state = create_game_state(Deck::Magic);
        assert!(has_voucher(&state, Voucher::CrystalBall));
        assert_eq!(state.consumables.len(), 2);
        assert_eq!(state.consumables[0], Consumable::Tarot(Tarot::Fool));
        assert_eq!(state.consumables[1], Consumable::Tarot(Tarot::Fool));
    }

    #[test]
    fn test_nebula_deck() {
        let state = create_game_state(Deck::Nebula);
        assert!(has_voucher(&state, Voucher::Telescope));
        assert_eq!(state.consumable_slots, 1); // base 2 - 1
    }

    #[test]
    fn test_ghost_deck() {
        let state = create_game_state(Deck::Ghost);
        assert_eq!(state.consumables.len(), 1);
        assert_eq!(state.consumables[0], Consumable::Spectral(Spectral::Hex));
    }
    #[test]
    fn test_abandoned_deck() {
        use crate::card::Rank;
        let state = create_game_state(Deck::Abandoned);
        assert_eq!(state.deck.len(), 40);

        let has_face_cards = state.deck.iter().any(|card| {
            let rank = (card.meta >> 12) & 0xF;
            rank == Rank::Jack as u16 || rank == Rank::Queen as u16 || rank == Rank::King as u16
        });
        assert!(!has_face_cards, "Abandoned deck should not have face cards");
    }

    #[test]
    fn test_checkered_deck() {
        use crate::card::Suit;
        let state = create_game_state(Deck::Checkered);
        assert_eq!(state.deck.len(), 52);

        let mut spades = 0;
        let mut hearts = 0;
        for card in state.deck.iter() {
            let suit = (card.meta >> 10) & 0x3;
            if suit == Suit::Spades as u16 {
                spades += 1;
            } else if suit == Suit::Hearts as u16 {
                hearts += 1;
            } else {
                panic!("Checkered deck should only have Spades and Hearts");
            }
        }
        assert_eq!(spades, 26);
        assert_eq!(hearts, 26);
    }
    #[test]
    fn test_zodiac_deck() {
        let state = create_game_state(Deck::Zodiac);
        assert!(has_voucher(&state, Voucher::TarotMerchant));
        assert!(has_voucher(&state, Voucher::PlanetMerchant));
        assert!(has_voucher(&state, Voucher::Overstock));
    }

    #[test]
    fn test_painted_deck() {
        let state = create_game_state(Deck::Painted);
        assert_eq!(state.hand_size, 10); // base 8 + 2
        assert_eq!(state.joker_slots, 4); // base 5 - 1
    }
}
