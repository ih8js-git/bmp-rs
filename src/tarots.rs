use crate::card::operations::{set_card_enhancement, set_card_rank, set_card_suit};
use crate::card::parse::parse_card_rank;
use crate::card::{Card, Enhancement, Rank, Suit};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tarot {
    Fool,
    Magician,
    HighPriestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    WheelOfFortune,
    Strength,
    HangedMan,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    World,
}

fn enhance_cards(
    cards: &mut Vec<Card>,
    num_of_allowed_cards: u8,
    enhancement: Enhancement,
) -> Result<&mut Vec<Card>, String> {
    if cards.is_empty() {
        return Err("No cards selected".to_string());
    }

    if cards.len() > num_of_allowed_cards as usize {
        return Err(format!("You can only use {} card(s)", num_of_allowed_cards));
    }

    for card in cards.iter_mut() {
        set_card_enhancement(card, enhancement);
    }

    Ok(cards)
}

fn change_cards_suit(cards: &mut Vec<Card>, suit: Suit) -> Result<&mut Vec<Card>, String> {
    if cards.is_empty() {
        return Err("No cards selected".to_string());
    }

    if cards.len() > 3 {
        return Err("You can only use 3 card".to_string());
    }

    for card in cards.iter_mut() {
        set_card_suit(card, suit);
    }

    Ok(cards)
}

fn use_enhancement_tarot(tarot: Tarot, cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    match tarot {
        Tarot::Magician => enhance_cards(cards, 2, Enhancement::Lucky),
        Tarot::Empress => enhance_cards(cards, 2, Enhancement::Mult),
        Tarot::Hierophant => enhance_cards(cards, 2, Enhancement::Bonus),
        Tarot::Lovers => enhance_cards(cards, 1, Enhancement::Wild),
        Tarot::Chariot => enhance_cards(cards, 1, Enhancement::Stone),
        Tarot::Justice => enhance_cards(cards, 1, Enhancement::Glass),
        Tarot::Devil => enhance_cards(cards, 1, Enhancement::Gold),
        Tarot::Tower => enhance_cards(cards, 1, Enhancement::Stone),
        _ => Err("Not a valid tarot for this function".to_string()),
    }
}

fn use_suit_tarot(tarot: Tarot, cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    match tarot {
        Tarot::Star => change_cards_suit(cards, Suit::Diamonds),
        Tarot::Moon => change_cards_suit(cards, Suit::Clubs),
        Tarot::Sun => change_cards_suit(cards, Suit::Hearts),
        Tarot::World => change_cards_suit(cards, Suit::Spades),
        _ => Err("Not a valid tarot for this function".to_string()),
    }
}

pub fn use_fool(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    return Err("Not coded yet".to_string());
}

pub fn use_high_priestess(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    return Err("Not coded yet".to_string());
}

pub fn use_emperor(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    return Err("Not coded yet".to_string());
}

pub fn use_wheel_of_fortune(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    return Err("Not coded yet".to_string());
}

pub fn use_strength(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    if cards.is_empty() {
        return Err("No cards selected".to_string());
    }

    if cards.len() > 2 {
        return Err("You can only use 2 cards".to_string());
    }

    for card in cards.iter_mut() {
        let rank = parse_card_rank(card);
        let next_rank = Rank::from_repr(rank as usize + 1).unwrap_or(Rank::Ace);
        set_card_rank(card, next_rank);
    }

    Ok(cards)
}

pub fn use_hanged_man(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    return Err("Not coded yet".to_string());
}

pub fn use_death(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    if cards.is_empty() {
        return Err("No cards selected".to_string());
    }

    if cards.len() != 2 {
        return Err("You must use 2 cards".to_string());
    }

    cards[1] = cards[0];

    Ok(cards)
}

pub fn use_temperance(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    return Err("Not coded yet".to_string());
}

pub fn use_judgement(cards: &mut Vec<Card>) -> Result<&mut Vec<Card>, String> {
    return Err("Not coded yet".to_string());
}
