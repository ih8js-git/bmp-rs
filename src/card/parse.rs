use crate::card::{Card, Rank, Suit, Edition, Enhancement, Seal, get_card_rank, get_card_suit, get_card_edition, get_card_enhancement, get_card_seal};

pub fn parse_card_to_text(card: &Card) -> String {
    let rank = get_card_rank(card);
    let suit = get_card_suit(card);
    let edition = get_card_edition(card);
    let enhancement = get_card_enhancement(card);
    let seal = get_card_seal(card);
    return format!(
        "{} {} {} {} of {}",
        parse_card_seal_to_text(seal),
        parse_card_edition_to_text(edition),
        parse_card_enhancement_to_text(enhancement),
        parse_card_rank_to_text(rank),
        parse_card_suit_to_text(suit),
    );
}

pub fn parse_card_rank_to_text(rank: Rank) -> String {
    match rank {
        Rank::Two => "2".to_string(),
        Rank::Three => "3".to_string(),
        Rank::Four => "4".to_string(),
        Rank::Five => "5".to_string(),
        Rank::Six => "6".to_string(),
        Rank::Seven => "7".to_string(),
        Rank::Eight => "8".to_string(),
        Rank::Nine => "9".to_string(),
        Rank::Ten => "10".to_string(),
        Rank::Jack => "J".to_string(),
        Rank::Queen => "Q".to_string(),
        Rank::King => "K".to_string(),
        Rank::Ace => "A".to_string(),
    }
}

pub fn parse_card_suit_to_text(suit: Suit) -> String {
    match suit {
        Suit::Spades => "Spades".to_string(),
        Suit::Hearts => "Hearts".to_string(),
        Suit::Clubs => "Clubs".to_string(),
        Suit::Diamonds => "Diamonds".to_string(),
    }
}

pub fn parse_card_edition_to_text(edition: Edition) -> String {
    match edition {
        Edition::None => "None".to_string(),
        Edition::Foil => "Foil".to_string(),
        Edition::Holographic => "Holographic".to_string(),
        Edition::Polychrome => "Polychrome".to_string(),
        Edition::Negative => "Negative".to_string(),
    }
}

pub fn parse_card_enhancement_to_text(enhancement: Enhancement) -> String {
    match enhancement {
        Enhancement::None => "None".to_string(),
        Enhancement::Bonus => "Bonus".to_string(),
        Enhancement::Mult => "Mult".to_string(),
        Enhancement::Wild => "Wild".to_string(),
        Enhancement::Glass => "Glass".to_string(),
        Enhancement::Steel => "Steel".to_string(),
        Enhancement::Stone => "Stone".to_string(),
        Enhancement::Gold => "Gold".to_string(),
        Enhancement::Lucky => "Lucky".to_string(),
    }
}

pub fn parse_card_seal_to_text(seal: Seal) -> String {
    match seal {
        Seal::None => "None".to_string(),
        Seal::Red => "Red Seal".to_string(),
        Seal::Blue => "Blue Seal".to_string(),
        Seal::Gold => "Gold Seal".to_string(),
        Seal::Purple => "Purple Seal".to_string(),
        _ => "Error".to_string(),
    }
}
