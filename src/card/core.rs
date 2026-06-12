use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/*
 * |Rank|Suit|Edition|Enhancement|Seal|
 * |----|----|-------|-----------|----|
 * |4b  |2b  |3b     |4b         |3b  |
 */

#[derive(Debug)]
pub struct Card {
    pub meta: u16,
    pub chips: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
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
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Edition {
    None,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
pub enum Seal {
    None,
    Gold,
    Red,
    Blue,
    Purple,
}

pub fn create_deck() -> Vec<Card> {
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
                Rank::Two => 2u16,
                Rank::Three => 3u16,
                Rank::Four => 4u16,
                Rank::Five => 5u16,
                Rank::Six => 6u16,
                Rank::Seven => 7u16,
                Rank::Eight => 8u16,
                Rank::Nine => 9u16,
                Rank::Ten => 10u16,
                Rank::Jack => 10u16,
                Rank::Queen => 10u16,
                Rank::King => 10u16,
                Rank::Ace => 11u16,
            };

            cards.push(Card { meta, chips });
        }
    }
    return cards;
}
