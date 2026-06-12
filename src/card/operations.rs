use crate::card::Card;

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

pub fn set_card_rank(card: &mut Card, rank: u8) {
    card.meta &= !(0b1111 << 12);                    // set prev rank bits to 0
    card.meta |= (rank as u16) << 12;           // insert new rank bits
}

pub fn set_card_suit(card: &mut Card, suit: u8) {
    let suit_mask = 0b11 << 10;
    card.meta &= !suit_mask; // set prev rank bits to 0
    card.meta |= (suit as u16) << 10; // insert new rank bits
}