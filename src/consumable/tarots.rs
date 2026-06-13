use crate::GameState;
use crate::card::operations::get_card_rank;
use crate::card::operations::{set_card_enhancement, set_card_rank, set_card_suit};
use crate::card::{Card, Enhancement, Rank, Suit};
use crate::consumable::Consumable;

/// Represents tarot card. Importantly The order of the enums is the same as
/// the order of the enhancements applied by the tarots (Hierophant through Magician),
/// allowing for optimization.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tarot {
    // make enum tarot and enhancement
    // pub enum Enhancement {
    Fool,       //    None,
    Hierophant, //    Bonus,
    Empress,    //    Mult,
    Lovers,     //    Wild,
    Justice,    //    Glass,
    Chariot,    //    Steel,
    Tower,      //    Stone,
    Devil,      //    Gold,
    Magician,   //    Lucky,
    HighPriestess,
    Emperor,
    Hermit,
    WheelOfFortune,
    Strength,
    HangedMan,
    Death,
    Temperance,
    Star,
    Moon,
    Sun,
    Judgement,
    World,
}

pub fn enhance_cards(
    cards: &mut [Card],
    num_of_allowed_cards: u8,
    enhancement: Enhancement,
) -> Result<(), String> {
    if cards.is_empty() {
        return Err("No cards selected".to_string());
    }

    if cards.len() > num_of_allowed_cards as usize {
        return Err(format!("You can only use {} cards", num_of_allowed_cards));
    }

    for card in cards.iter_mut() {
        set_card_enhancement(card, enhancement);
    }

    Ok(())
}

pub fn change_cards_suit(cards: &mut [Card], suit: Suit) -> Result<(), String> {
    if cards.is_empty() {
        return Err("No cards selected".to_string());
    }

    if cards.len() > 3 {
        return Err("You can only use 3 card".to_string());
    }

    for card in cards.iter_mut() {
        set_card_suit(card, suit);
    }

    Ok(())
}

pub fn use_tarot(
    game_state: &mut GameState,
    tarot: Tarot,
    selected_cards: &mut [Card],
) -> Result<(), String> {
    game_state.tarots_used += 1;
    game_state.last_used = Consumable::Tarot(tarot);

    match tarot {
        Tarot::Fool => use_fool(game_state),
        Tarot::Magician => enhance_cards(selected_cards, 2, Enhancement::Lucky),
        Tarot::HighPriestess => use_high_priestess(selected_cards),
        Tarot::Empress => enhance_cards(selected_cards, 2, Enhancement::Mult),
        Tarot::Emperor => use_emperor(selected_cards),
        Tarot::Hierophant => enhance_cards(selected_cards, 2, Enhancement::Bonus),
        Tarot::Lovers => enhance_cards(selected_cards, 1, Enhancement::Wild),
        Tarot::Chariot => enhance_cards(selected_cards, 1, Enhancement::Stone),
        Tarot::Justice => enhance_cards(selected_cards, 1, Enhancement::Glass),
        Tarot::Hermit => use_hermit(selected_cards),
        Tarot::WheelOfFortune => use_wheel_of_fortune(selected_cards),
        Tarot::Strength => use_strength(selected_cards),
        Tarot::HangedMan => use_hanged_man(selected_cards),
        Tarot::Death => use_death(selected_cards),
        Tarot::Temperance => use_temperance(selected_cards),
        Tarot::Devil => enhance_cards(selected_cards, 1, Enhancement::Gold),
        Tarot::Tower => enhance_cards(selected_cards, 1, Enhancement::Stone),
        Tarot::Star => change_cards_suit(selected_cards, Suit::Diamonds),
        Tarot::Moon => change_cards_suit(selected_cards, Suit::Clubs),
        Tarot::Sun => change_cards_suit(selected_cards, Suit::Hearts),
        Tarot::Judgement => use_judgement(selected_cards),
        Tarot::World => change_cards_suit(selected_cards, Suit::Spades),
    }
}

pub fn use_fool(game_state: &mut GameState) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_high_priestess(cards: &mut [Card]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_emperor(cards: &mut [Card]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_hermit(cards: &mut [Card]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_wheel_of_fortune(cards: &mut [Card]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_strength(cards: &mut [Card]) -> Result<(), String> {
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

    Ok(())
}

pub fn use_hanged_man(cards: &mut [Card]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_death(cards: &mut [Card]) -> Result<(), String> {
    if cards.is_empty() {
        return Err("No cards selected".to_string());
    }

    if cards.len() != 2 {
        return Err("You must use 2 cards".to_string());
    }

    cards[1] = cards[0];

    Ok(())
}

pub fn use_temperance(cards: &mut [Card]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_judgement(cards: &mut [Card]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameState;
    use crate::card::core::create_test_card;
    use crate::card::operations::{get_card_enhancement, get_card_rank, get_card_suit};
    use crate::consumable::Consumable;

    fn dummy_game_state() -> GameState {
        GameState {
            last_used: Consumable::Tarot(Tarot::Fool),
            tarots_used: 0,
            deck: vec![],
        }
    }

    fn use_enhancement_tarot(tarot: Tarot, cards: &mut [Card]) -> Result<(), String> {
        use_tarot(&mut dummy_game_state(), tarot, cards)
    }

    fn use_suit_tarot(tarot: Tarot, cards: &mut [Card]) -> Result<(), String> {
        use_tarot(&mut dummy_game_state(), tarot, cards)
    }

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
