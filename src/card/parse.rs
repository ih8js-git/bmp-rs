use crate::card::{Card, Enhancement, Rank, Suit};

pub fn get_card_rank(card: &Card) -> Rank {
    match card.meta >> 12 {
        0 => Rank::Two,
        1 => Rank::Three,
        2 => Rank::Four,
        3 => Rank::Five,
        4 => Rank::Six,
        5 => Rank::Seven,
        6 => Rank::Eight,
        7 => Rank::Nine,
        8 => Rank::Ten,
        9 => Rank::Jack,
        10 => Rank::Queen,
        11 => Rank::King,
        12 => Rank::Ace,
        _ => panic!("Invalid rank"),
    }
}

pub fn get_card_suit(card: &Card) -> Suit {
    let suit = (card.meta & 0b11_000_0000_000) >> 10;
    match suit {
        0 => Suit::Spades,
        1 => Suit::Hearts,
        2 => Suit::Clubs,
        3 => Suit::Diamonds,
        _ => panic!("Invalid suit"),
    }
}

pub fn get_card_enhancement(card: &Card) -> Enhancement {
    let enhancement = (card.meta & 0b1111_000) >> 3;
    match enhancement {
        0 => Enhancement::None,
        1 => Enhancement::Bonus,
        2 => Enhancement::Mult,
        3 => Enhancement::Wild,
        4 => Enhancement::Glass,
        5 => Enhancement::Steel,
        6 => Enhancement::Stone,
        7 => Enhancement::Gold,
        8 => Enhancement::Lucky,
        _ => panic!("Invalid enhancement"),
    }
}

pub fn parse_card_to_text(card: &Card) -> String {
    let rank = card.meta >> 12;
    let suit = (card.meta & 0b11_000_0000_000) >> 10;
    let edition = (card.meta & 0b111_000_000) >> 7;
    let enhancement = (card.meta & 0b1111_000) >> 3;
    let seal = card.meta & 0b111;
    return format!(
        "{} {} {} {} of {}",
        parse_card_seal_to_text(seal as u8),
        parse_card_edition_to_text(edition as u8),
        parse_card_enhancement_to_text(enhancement as u8),
        parse_card_rank_to_text(rank as u8),
        parse_card_suit_to_text(suit as u8),
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

pub fn parse_card_suit_to_text(suit: u8) -> String {
    match suit {
        0 => "Spades".to_string(),
        1 => "Hearts".to_string(),
        2 => "Clubs".to_string(),
        3 => "Diamonds".to_string(),
        _ => "Error".to_string(),
    }
}

pub fn parse_card_edition_to_text(edition: u8) -> String {
    match edition {
        0 => "None".to_string(),
        1 => "Foil".to_string(),
        2 => "Holographic".to_string(),
        3 => "Polychrome".to_string(),
        4 => "Negative".to_string(),
        _ => "Error".to_string(),
    }
}

pub fn parse_card_enhancement_to_text(enhancement: u8) -> String {
    match enhancement {
        0 => "None".to_string(),
        1 => "Bonus".to_string(),
        2 => "Mult".to_string(),
        3 => "Wild".to_string(),
        4 => "Glass".to_string(),
        5 => "Steel".to_string(),
        6 => "Stone".to_string(),
        7 => "Gold".to_string(),
        8 => "Lucky".to_string(),
        _ => "Error".to_string(),
    }
}

pub fn parse_card_seal_to_text(seal: u8) -> String {
    match seal {
        0 => "None".to_string(),
        1 => "Red Seal".to_string(),
        2 => "Blue Seal".to_string(),
        3 => "Gold Seal".to_string(),
        4 => "Purple Seal".to_string(),
        _ => "Error".to_string(),
    }
}
