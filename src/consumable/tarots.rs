use crate::card::operations::get_card_rank;
use crate::card::operations::{set_card_enhancement, set_card_rank, set_card_suit};
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
        let rank = get_card_rank(card);
        let next_rank = Rank::from_repr(rank as u8 + 1).unwrap_or(Rank::Ace);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::create_test_card;
    use crate::card::operations::{get_card_enhancement, get_card_rank, get_card_suit};

    #[test]
    fn test_tarot_strength() {
        let mut cards = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
        ];

        let result = use_strength(&mut cards);
        assert!(result.is_ok());

        assert_eq!(get_card_rank(&cards[0]), Rank::Three);
        assert_eq!(get_card_rank(&cards[1]), Rank::Ace);
    }

    #[test]
    fn test_tarot_strength_ace() {
        let mut cards = vec![create_test_card(Rank::Ace, Suit::Spades)];
        let result = use_strength(&mut cards);
        assert!(result.is_ok());
        assert_eq!(get_card_rank(&cards[0]), Rank::Ace);
    }

    #[test]
    fn test_tarot_death() {
        let mut cards = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
        ];

        let result = use_death(&mut cards);
        assert!(result.is_ok());

        assert_eq!(get_card_rank(&cards[0]), Rank::Two);
        assert_eq!(get_card_suit(&cards[0]), Suit::Spades);
        assert_eq!(get_card_rank(&cards[1]), Rank::Two);
        assert_eq!(get_card_suit(&cards[1]), Suit::Spades);
    }

    #[test]
    fn test_enhancement_tarots() {
        let test_cases = vec![
            (Tarot::Magician, Enhancement::Lucky, 2),
            (Tarot::Empress, Enhancement::Mult, 2),
            (Tarot::Hierophant, Enhancement::Bonus, 2),
            (Tarot::Lovers, Enhancement::Wild, 1),
            (Tarot::Chariot, Enhancement::Stone, 1),
            (Tarot::Justice, Enhancement::Glass, 1),
            (Tarot::Devil, Enhancement::Gold, 1),
            (Tarot::Tower, Enhancement::Stone, 1),
        ];

        for (tarot, expected_enhancement, max_cards) in test_cases {
            let mut cards = vec![create_test_card(Rank::Two, Suit::Spades); max_cards];
            let result = use_enhancement_tarot(tarot, &mut cards);
            assert!(result.is_ok());
            for card in &cards {
                assert_eq!(get_card_enhancement(card), expected_enhancement);
            }
        }
    }

    #[test]
    fn test_suit_tarots() {
        let test_cases = vec![
            (Tarot::Star, Suit::Diamonds),
            (Tarot::Moon, Suit::Clubs),
            (Tarot::Sun, Suit::Hearts),
            (Tarot::World, Suit::Spades),
        ];

        for (tarot, expected_suit) in test_cases {
            let mut cards = vec![
                create_test_card(Rank::Two, Suit::Spades),
                create_test_card(Rank::Three, Suit::Hearts),
                create_test_card(Rank::Four, Suit::Clubs),
            ];
            let result = use_suit_tarot(tarot, &mut cards);
            assert!(result.is_ok());
            for card in &cards {
                assert_eq!(get_card_suit(card), expected_suit);
            }
        }
    }
}
