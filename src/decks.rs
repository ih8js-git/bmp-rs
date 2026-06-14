use crate::GameState;
use crate::Voucher;
use crate::card::{Card, Edition, Enhancement, Rank, Seal, Suit};
use crate::consumable::Spectral;
use crate::consumable::{Consumable, Tarot};
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

fn create_default_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            let current_rank = rank;
            let current_suit = suit;
            let edition = Edition::None;
            let enhancement = Enhancement::None;
            let seal = Seal::None;
            let meta = (current_rank as u16) << 12
                | (current_suit as u16) << 10
                | (edition as u16) << 7
                | (enhancement as u16) << 3
                | (seal as u16);

            let chips: u16 = match current_rank {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 10,
                Rank::Queen => 10,
                Rank::King => 10,
                Rank::Ace => 11,
            };

            cards.push(Card { meta, chips });
        }
    }
    return cards;
}

fn create_abandoned_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            let current_rank = rank;
            let current_suit = suit;
            let edition = Edition::None;
            let enhancement = Enhancement::None;
            let seal = Seal::None;
            let meta = (current_rank as u16) << 12
                | (current_suit as u16) << 10
                | (edition as u16) << 7
                | (enhancement as u16) << 3
                | (seal as u16);

            let chips: u16 = match current_rank {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 10,
                Rank::Queen => 10,
                Rank::King => 10,
                Rank::Ace => 11,
            };

            if current_rank != Rank::Jack
                && current_rank != Rank::Queen
                && current_rank != Rank::King
            {
                cards.push(Card { meta, chips });
            }
        }
    }
    return cards;
}

fn create_checkered_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            let current_rank = rank;
            let mut current_suit = suit;
            if current_suit == Suit::Clubs {
                current_suit = Suit::Spades;
            } else if current_suit == Suit::Diamonds {
                current_suit = Suit::Hearts;
            }
            let edition = Edition::None;
            let enhancement = Enhancement::None;
            let seal = Seal::None;
            let meta = (current_rank as u16) << 12
                | (current_suit as u16) << 10
                | (edition as u16) << 7
                | (enhancement as u16) << 3
                | (seal as u16);

            let chips: u16 = match current_rank {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 10,
                Rank::Queen => 10,
                Rank::King => 10,
                Rank::Ace => 11,
            };

            cards.push(Card { meta, chips });
        }
    }
    return cards;
}

pub fn create_game_state(deck: Deck) -> GameState {
    let base = GameState {
        last_used: Consumable::Tarot(Tarot::Fool),
        tarots_used: 0,
        deck: Vec::new(),
        vouchers: Vec::with_capacity(8),
        hand: Vec::with_capacity(8),
        hand_size: 8,
        jokers: Vec::with_capacity(5),
        joker_slots: 5,
        consumables: Vec::with_capacity(2),
        consumable_slots: 2,
        balance: 4,
        hands: 4,
        discards: 3,
    };

    match deck {
        Deck::Red => GameState {
            deck: create_default_deck(),
            discards: base.discards + 1,
            ..base
        },
        Deck::Blue => GameState {
            deck: create_default_deck(),
            hands: base.hands + 1,
            ..base
        },
        Deck::Yellow => GameState {
            deck: create_default_deck(),
            balance: base.balance + 10,
            ..base
        },
        Deck::Black => GameState {
            deck: create_default_deck(),
            joker_slots: base.joker_slots + 1,
            hands: base.hands - 1,
            ..base
        },
        Deck::Magic => GameState {
            deck: create_default_deck(),
            vouchers: vec![Voucher::CrystalBall],
            consumables: vec![
                Consumable::Tarot(Tarot::Fool),
                Consumable::Tarot(Tarot::Fool),
            ],
            ..base
        },
        Deck::Nebula => GameState {
            deck: create_default_deck(),
            vouchers: vec![Voucher::Telescope],
            consumable_slots: base.consumable_slots - 1,
            ..base
        },
        Deck::Ghost => GameState {
            deck: create_default_deck(),
            consumables: vec![Consumable::Spectral(Spectral::Hex)],
            ..base
        },
        Deck::Abandoned => GameState {
            deck: create_abandoned_deck(),
            ..base
        },
        Deck::Checkered => GameState {
            deck: create_checkered_deck(),
            ..base
        },
        Deck::Zodiac => GameState {
            deck: create_default_deck(),
            vouchers: vec![
                Voucher::TarotMerchant,
                Voucher::PlanetMerchant,
                Voucher::Overstock,
            ],
            ..base
        },
        Deck::Painted => GameState {
            deck: create_default_deck(),
            hand_size: base.hand_size + 2,
            joker_slots: base.joker_slots - 1,
            ..base
        },
        _ => GameState {
            deck: create_default_deck(),
            ..base
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Voucher;
    use crate::consumable::{Consumable, Spectral, Tarot};

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
        assert!(state.vouchers.is_empty());
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
        assert_eq!(state.vouchers.len(), 1);
        assert_eq!(state.vouchers[0], Voucher::CrystalBall);
        assert_eq!(state.consumables.len(), 2);
        assert_eq!(state.consumables[0], Consumable::Tarot(Tarot::Fool));
        assert_eq!(state.consumables[1], Consumable::Tarot(Tarot::Fool));
    }

    #[test]
    fn test_nebula_deck() {
        let state = create_game_state(Deck::Nebula);
        assert_eq!(state.vouchers.len(), 1);
        assert_eq!(state.vouchers[0], Voucher::Telescope);
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
        assert_eq!(state.vouchers.len(), 3);
        assert_eq!(state.vouchers[0], Voucher::TarotMerchant);
        assert_eq!(state.vouchers[1], Voucher::PlanetMerchant);
        assert_eq!(state.vouchers[2], Voucher::Overstock);
    }

    #[test]
    fn test_painted_deck() {
        let state = create_game_state(Deck::Painted);
        assert_eq!(state.hand_size, 10); // base 8 + 2
        assert_eq!(state.joker_slots, 4); // base 5 - 1
    }
}
