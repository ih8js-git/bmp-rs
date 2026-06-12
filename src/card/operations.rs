use crate::card::{Card, Edition, Enhancement, Rank, Seal, Suit};

/*
 * |Rank|Suit|Edition|Enhancement|Seal|
 * |----|----|-------|-----------|----|
 * |4b  |2b  |3b     |4b         |3b  |
 * |    |    |       |           |    |
 * |  12|  10|      7|          3|   0|
 */

pub fn set_card_chips(card: &mut Card, chips: u16) {
    card.chips = chips;
}

pub fn set_card_rank(card: &mut Card, rank: Rank) {
    card.meta &= !(0b1111 << 12);                   // set prev rank bits to 0
    card.meta |= (rank as u16) << 12;               // insert new rank bits
}

pub fn set_card_suit(card: &mut Card, suit: Suit) {
    card.meta &= !(0b11 << 10);
    card.meta |= (suit as u16) << 10;
}

pub fn set_card_edition(card: &mut Card, edition: Edition) {
    card.meta &= !(0b111 << 7);
    card.meta |= (edition as u16) << 7;
}

pub fn set_card_enhancement(card: &mut Card, enhancement: Enhancement) {
    card.meta &= !(0b1111 << 3);
    card.meta |= (enhancement as u16) << 3;
}

pub fn set_card_seal(card: &mut Card, seal: Seal) {
    card.meta &= !0b111;
    card.meta |= seal as u16;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a clean card
    fn create_blank_card() -> Card {
        Card {
            meta: 0,
            chips: 0,
        }
    }

    #[test]
    fn test_set_card_chips() {
        let mut card = create_blank_card();

        set_card_chips(&mut card, 1337);
        assert_eq!(card.chips, 1337);

        set_card_chips(&mut card, u16::MAX);
        assert_eq!(card.chips, u16::MAX);
    }

    #[test]
    fn test_set_card_rank() {
        let mut card = create_blank_card();
        let target_rank = Rank::Ace; // Assumes your enum has an Ace variant

        set_card_rank(&mut card, target_rank);
        assert_eq!(card.meta, (target_rank as u16) << 12);

        // Test clearing logic with a different variant
        let new_rank = Rank::Two;
        set_card_rank(&mut card, new_rank);
        assert_eq!(card.meta, (new_rank as u16) << 12);
    }

    #[test]
    fn test_set_card_suit() {
        let mut card = create_blank_card();
        let target_suit = Suit::Spades;

        set_card_suit(&mut card, target_suit);
        assert_eq!(card.meta, (target_suit as u16) << 10);

        let new_suit = Suit::Hearts;
        set_card_suit(&mut card, new_suit);
        assert_eq!(card.meta, (new_suit as u16) << 10);
    }

    #[test]
    fn test_set_card_edition() {
        let mut card = create_blank_card();
        let target_edition = Edition::Foil;

        set_card_edition(&mut card, target_edition);
        assert_eq!(card.meta, (target_edition as u16) << 7);

        let new_edition = Edition::Holographic;
        set_card_edition(&mut card, new_edition);
        assert_eq!(card.meta, (new_edition as u16) << 7);
    }

    #[test]
    fn test_set_card_enhancement() {
        let mut card = create_blank_card();
        let target_enhancement = Enhancement::Mult;

        set_card_enhancement(&mut card, target_enhancement);
        assert_eq!(card.meta, (target_enhancement as u16) << 3);

        let new_enhancement = Enhancement::Bonus;
        set_card_enhancement(&mut card, new_enhancement);
        assert_eq!(card.meta, (new_enhancement as u16) << 3);
    }

    #[test]
    fn test_set_card_seal() {
        let mut card = create_blank_card();
        let target_seal = Seal::Gold;

        set_card_seal(&mut card, target_seal);
        assert_eq!(card.meta, (target_seal as u16) << 0);

        let new_seal = Seal::Red;
        set_card_seal(&mut card, new_seal);
        assert_eq!(card.meta, (new_seal as u16) << 0);
    }

    #[test]
    fn test_all_bit_fields_packed_simultaneously() {
        let mut card = create_blank_card();

        // Choose arbitrary enum variants to represent a fully decorated card
        let rank = Rank::King;
        let suit = Suit::Diamonds;
        let edition = Edition::Polychrome;
        let enhancement = Enhancement::Lucky;
        let seal = Seal::Purple;

        // Set everything sequentially
        set_card_rank(&mut card, rank);
        set_card_suit(&mut card, suit);
        set_card_edition(&mut card, edition);
        set_card_enhancement(&mut card, enhancement);
        set_card_seal(&mut card, seal);

        // Construct our expected binary map mathematically
        let expected_meta = ((rank as u16) << 12)
            | ((suit as u16) << 10)
            | ((edition as u16) << 7)
            | ((enhancement as u16) << 3)
            | ((seal as u16) << 0);

        // Verify no bits overwrote or corrupted adjacent neighbors
        assert_eq!(card.meta, expected_meta);
    }
}