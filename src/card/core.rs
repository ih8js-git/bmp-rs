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
            });
        }
    }
    return cards;
}
