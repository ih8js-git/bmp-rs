/*
 * |Rank|Suit|Edition|Enhancement|Seal|
 * |----|----|-------|-----------|----|
 * |4b  |2b  |3b     |4b         |3b  |
 */
#[derive(Debug)]
pub struct Card {
    pub meta: u16,
    pub chips: u16
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    Ace
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    for suit in 0..4 {
        for rank in 0..13 {
            let current_rank = rank;
            let current_suit = suit;
            let edition = 0;
            let enhancement = 0;
            let seal = 0;
            let meta =
                current_rank << 12 | current_suit << 10 | edition << 7 | enhancement << 3 | seal;

            let chips: u16 = match rank {
                0 => 2u16,
                1 => 3u16,
                2 => 4u16,
                3 => 5u16,
                4 => 6u16,
                5 => 7u16,
                6 => 8u16,
                7 => 9u16,
                8 => 10u16,
                9 => 10u16,
                10 => 10u16,
                11 => 10u16,
                12 => 11u16,
                _ => unreachable!(),
            };

            cards.push(Card {
                meta,
                chips,
            });
        }
    }
    return cards;
}
