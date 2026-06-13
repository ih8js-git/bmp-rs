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
    card.meta &= !(0b1111 << 12); // set prev rank bits to 0
    card.meta |= (rank as u16) << 12; // insert new rank bits
}

pub fn get_card_rank(card: &Card) -> Rank {
    let rank = (card.meta >> 12) as u8;
    unsafe { std::mem::transmute(rank) } // We guarantee rank will be within enum boundaries
}

pub fn set_card_suit(card: &mut Card, suit: Suit) {
    card.meta &= !(0b11 << 10); // set prev suit bits to 0
    card.meta |= (suit as u16) << 10; // insert new suit bits
}

pub fn get_card_suit(card: &Card) -> Suit {
    let suit = ((card.meta >> 10) & 0b11) as u8;
    unsafe { std::mem::transmute(suit) } // We guarantee suit will be within enum boundaries
}

pub fn set_card_edition(card: &mut Card, edition: Edition) {
    card.meta &= !(0b111 << 7);
    card.meta |= (edition as u16) << 7;
}

pub fn get_card_edition(card: &Card) -> Edition {
    let edition = ((card.meta >> 7) & 0b111) as u8;
    unsafe { std::mem::transmute(edition) } // We guarantee edition will be within enum boundaries
}

pub fn set_card_enhancement(card: &mut Card, enhancement: Enhancement) {
    card.meta &= !(0b1111 << 3); // set prev enhancement bits to 0
    card.meta |= (enhancement as u16) << 3; // insert enhancement suit bits
}

pub fn get_card_enhancement(card: &Card) -> Enhancement {
    let enhancement = ((card.meta >> 3) & 0b1111) as u8;
    unsafe { std::mem::transmute(enhancement) } // We guarantee enhancement will be within enum boundaries
}

pub fn set_card_seal(card: &mut Card, seal: Seal) {
    card.meta &= !0b111; // set seal bits to 0
    card.meta |= seal as u16; // insert seal suit bits
}

pub fn get_card_seal(card: &Card) -> Seal {
    let seal = (card.meta & 0b111) as u8;
    unsafe { std::mem::transmute(seal) } // We guarantee seal will be within enum boundaries
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a clean card
    fn create_blank_card() -> Card {
        Card { meta: 0, chips: 0 }
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

    #[test]
    fn test_get_card_rank() {
        let mut card = create_blank_card();
        let target_rank = Rank::Ace;

        set_card_rank(&mut card, target_rank);
        assert_eq!(get_card_rank(&card), target_rank);
    }

    #[test]
    fn test_get_card_suit() {
        let mut card = create_blank_card();
        let target_suit = Suit::Spades;

        set_card_suit(&mut card, target_suit);
        assert_eq!(get_card_suit(&card), target_suit);
    }

    #[test]
    fn test_get_card_edition() {
        let mut card = create_blank_card();
        let target_edition = Edition::Foil;

        set_card_edition(&mut card, target_edition);
        assert_eq!(get_card_edition(&card), target_edition);
    }

    #[test]
    fn test_get_card_enhancement() {
        let mut card = create_blank_card();
        let target_enhancement = Enhancement::Mult;

        set_card_enhancement(&mut card, target_enhancement);
        assert_eq!(get_card_enhancement(&card), target_enhancement);
    }

    #[test]
    fn test_get_card_seal() {
        let mut card = create_blank_card();
        let target_seal = Seal::Gold;

        set_card_seal(&mut card, target_seal);
        assert_eq!(get_card_seal(&card), target_seal);
    }

    #[test]
    fn test_getters_with_fully_packed_card() {
        let mut card = create_blank_card();

        // 1. Pack a highly specific combination of bits
        let expected_rank = Rank::King;
        let expected_suit = Suit::Diamonds;
        let expected_edition = Edition::Polychrome;
        let expected_enhancement = Enhancement::Lucky;
        let expected_seal = Seal::Purple;

        set_card_rank(&mut card, expected_rank);
        set_card_suit(&mut card, expected_suit);
        set_card_edition(&mut card, expected_edition);
        set_card_enhancement(&mut card, expected_enhancement);
        set_card_seal(&mut card, expected_seal);

        // 2. Unpack everything using getters to verify zero bit-bleeding
        assert_eq!(get_card_rank(&card), expected_rank);
        assert_eq!(get_card_suit(&card), expected_suit);
        assert_eq!(get_card_edition(&card), expected_edition);
        assert_eq!(get_card_enhancement(&card), expected_enhancement);
        assert_eq!(get_card_seal(&card), expected_seal);
    }
}
