use crate::card::operations;
use modular_bitfield::prelude::*;
use std::fmt;

use strum_macros::{EnumCount, EnumIter, FromRepr};

/*
 * |Rank|Suit|Edition|Enhancement|Seal|
 * |----|----|-------|-----------|----|
 * |4b  |2b  |3b     |4b         |3b  |
 */

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Flags {
    played_this_ante: bool,
    destroyed: bool,
    #[skip]
    __: B6,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    pub meta: u16,
    pub chips: u16,
    pub id: u16,
    pub flags: Flags,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let enhancement = operations::get_card_enhancement(self);
        let seal = operations::get_card_seal(self);
        let edition = operations::get_card_edition(self);
        let rank = operations::get_card_rank(self);
        let suit = operations::get_card_suit(self);

        if edition != Edition::None {
            write!(f, "{edition} ")?;
        }
        if enhancement != Enhancement::None {
            write!(f, "{enhancement} ")?;
        }
        if seal != Seal::None {
            write!(f, "{seal} Seal ")?;
        }
        write!(f, "{rank} of {suit}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, FromRepr, strum_macros::Display)]
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

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, strum_macros::Display)]
#[repr(u8)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumCount, strum_macros::Display)]
#[repr(u8)]
pub enum Edition {
    None,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

impl Edition {
    pub const fn added_cost(&self) -> u8 {
        match self {
            Edition::None => 0,
            Edition::Foil => 2,
            Edition::Holographic => 3,
            Edition::Polychrome => 5,
            Edition::Negative => 5,
        }
    }
}

/// Represents an enhancement on a card. Importantly The order of the enums is the same as
/// the order of the tarots applying this enhancement, allowing for optimization.
#[derive(Debug, Copy, Clone, PartialEq, strum_macros::Display)]
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

#[derive(Debug, Copy, Clone, PartialEq, strum_macros::Display)]
#[repr(u8)]
pub enum Seal {
    None,
    Gold,
    Red,
    Blue,
    Purple,
}

pub fn create_card(rank: Rank, suit: Suit) -> Card {
    create_card_with_id(rank, suit, 0)
}

pub fn create_card_with_id(rank: Rank, suit: Suit, id: u16) -> Card {
    let meta = (rank as u16) << 12
        | (suit as u16) << 10
        | (Edition::None as u16) << 7
        | (Enhancement::None as u16) << 3
        | (Seal::None as u16);

    let chips: u16 = match rank {
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

    Card {
        meta,
        chips,
        id,
        flags: Flags::new(),
    }
}
