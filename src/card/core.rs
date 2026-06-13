use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};

/*
 * |Rank|Suit|Edition|Enhancement|Seal|
 * |----|----|-------|-----------|----|
 * |4b  |2b  |3b     |4b         |3b  |
 */

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub meta: u16,
    pub chips: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, FromRepr)]
#[repr(u8)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
#[repr(u8)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Edition {
    None,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

/// Represents an enhancement on a card. Importantly The order of the enums is the same as
/// the order of the tarots applying this enhancement, allowing for optimization.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Enhancement {
    None,
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Seal {
    None,
    Gold,
    Red,
    Blue,
    Purple,
}

pub fn create_test_card(rank: Rank, suit: Suit) -> Card {
    let meta = (rank as u16) << 12
        | (suit as u16) << 10
        | (Edition::None as u16) << 7
        | (Enhancement::None as u16) << 3
        | (Seal::None as u16);
    Card { meta, chips: 0 }
}
