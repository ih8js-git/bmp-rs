/*
 * |Rank|Suit|Edition|Enhancement|Seal|
 * |----|----|-------|-----------|----|
 * |4b  |2b  |3b     |4b         |3b  |
 */
#[derive(Debug)]
pub struct Card {
    meta: u16,
    chips: u16,
    id: u16,
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
                0 => 2 as u16,
                1 => 3 as u16,
                2 => 4 as u16,
                3 => 5 as u16,
                4 => 6 as u16,
                5 => 7 as u16,
                6 => 8 as u16,
                7 => 9 as u16,
                8 => 10 as u16,
                9 => 10 as u16,
                10 => 10 as u16,
                11 => 10 as u16,
                12 => 11 as u16,
                _ => unreachable!(),
            };

            cards.push(Card {
                meta: meta,
                chips: chips,
                id: (suit * 13 + rank) as u16,
            });
        }
    }
    return cards;
}

pub fn parse_card_to_text(card: &Card) -> String {
    let rank = card.meta >> 12;
    let suit = (card.meta & 0b11_000_0000_000) >> 10;
    return format!(
        "{} of {}",
        parse_card_rank_to_text(rank as u8),
        parse_suit_to_text(suit as u8)
    );
}

pub fn parse_card_rank_to_text(rank: u8) -> String {
    match rank {
        0 => "2".to_string(),
        1 => "3".to_string(),
        2 => "4".to_string(),
        3 => "5".to_string(),
        4 => "6".to_string(),
        5 => "7".to_string(),
        6 => "8".to_string(),
        7 => "9".to_string(),
        8 => "10".to_string(),
        9 => "J".to_string(),
        10 => "Q".to_string(),
        11 => "K".to_string(),
        12 => "A".to_string(),
        _ => "Error".to_string(),
    }
}

pub fn parse_suit_to_text(suit: u8) -> String {
    match suit {
        0 => "Spades".to_string(),
        1 => "Hearts".to_string(),
        2 => "Clubs".to_string(),
        3 => "Diamonds".to_string(),
        _ => "Error".to_string(),
    }
}

pub fn parse_enhancement_to_text(enhancement: u8) -> String {}
