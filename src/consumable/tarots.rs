use crate::GameState;
use crate::card::operations::get_card_rank;
use crate::card::operations::{set_card_enhancement, set_card_rank, set_card_suit};
use crate::card::{Card, Enhancement, Rank, Suit};
use crate::consumable::Consumable;

/// Represents tarot card. Importantly The order of the enums is the same as
/// the order of the enhancements applied by the tarots (Hierophant through Magician),
/// allowing for optimization.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
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
    hand: &mut [Card],
    indices: &[usize],
    num_of_allowed_cards: u8,
    enhancement: Enhancement,
) -> Result<(), String> {
    if indices.is_empty() {
        return Err("No cards selected".to_string());
    }

    if indices.len() > num_of_allowed_cards as usize {
        return Err(format!("You can only use {} cards", num_of_allowed_cards));
    }

    for &idx in indices {
        set_card_enhancement(&mut hand[idx], enhancement);
    }

    Ok(())
}

pub fn change_cards_suit(hand: &mut [Card], indices: &[usize], suit: Suit) -> Result<(), String> {
    if indices.is_empty() {
        return Err("No cards selected".to_string());
    }

    if indices.len() > 3 {
        return Err("You can only use 3 cards".to_string());
    }

    for &idx in indices {
        set_card_suit(&mut hand[idx], suit);
    }

    Ok(())
}

pub fn use_tarot(
    game_state: &mut GameState,
    tarot: Tarot,
    selected_indices: &[usize],
) -> Result<(), String> {
    game_state.tarots_used += 1;
    game_state.last_used = Consumable::Tarot(tarot);

    match tarot {
        Tarot::Fool => use_fool(game_state),
        Tarot::Magician => enhance_cards(
            &mut game_state.hand,
            selected_indices,
            2,
            Enhancement::Lucky,
        ),
        Tarot::HighPriestess => use_high_priestess(&mut game_state.hand, selected_indices),
        Tarot::Empress => {
            enhance_cards(&mut game_state.hand, selected_indices, 2, Enhancement::Mult)
        }
        Tarot::Emperor => use_emperor(&mut game_state.hand, selected_indices),
        Tarot::Hierophant => enhance_cards(
            &mut game_state.hand,
            selected_indices,
            2,
            Enhancement::Bonus,
        ),
        Tarot::Lovers => {
            enhance_cards(&mut game_state.hand, selected_indices, 1, Enhancement::Wild)
        }
        Tarot::Chariot => enhance_cards(
            &mut game_state.hand,
            selected_indices,
            1,
            Enhancement::Stone,
        ),
        Tarot::Justice => enhance_cards(
            &mut game_state.hand,
            selected_indices,
            1,
            Enhancement::Glass,
        ),
        Tarot::Hermit => use_hermit(&mut game_state.hand, selected_indices),
        Tarot::WheelOfFortune => use_wheel_of_fortune(&mut game_state.hand, selected_indices),
        Tarot::Strength => use_strength(&mut game_state.hand, selected_indices),
        Tarot::HangedMan => use_hanged_man(&mut game_state.hand, selected_indices),
        Tarot::Death => use_death(&mut game_state.hand, selected_indices),
        Tarot::Temperance => use_temperance(&mut game_state.hand, selected_indices),
        Tarot::Devil => enhance_cards(&mut game_state.hand, selected_indices, 1, Enhancement::Gold),
        Tarot::Tower => enhance_cards(
            &mut game_state.hand,
            selected_indices,
            1,
            Enhancement::Stone,
        ),
        Tarot::Star => change_cards_suit(&mut game_state.hand, selected_indices, Suit::Diamonds),
        Tarot::Moon => change_cards_suit(&mut game_state.hand, selected_indices, Suit::Clubs),
        Tarot::Sun => change_cards_suit(&mut game_state.hand, selected_indices, Suit::Hearts),
        Tarot::Judgement => use_judgement(&mut game_state.hand, selected_indices),
        Tarot::World => change_cards_suit(&mut game_state.hand, selected_indices, Suit::Spades),
    }
}

pub fn use_fool(game_state: &mut GameState) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_high_priestess(hand: &mut [Card], indices: &[usize]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_emperor(hand: &mut [Card], indices: &[usize]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_hermit(hand: &mut [Card], indices: &[usize]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_wheel_of_fortune(hand: &mut [Card], indices: &[usize]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_strength(hand: &mut [Card], indices: &[usize]) -> Result<(), String> {
    if indices.is_empty() {
        return Err("No cards selected".to_string());
    }

    if indices.len() > 2 {
        return Err("You can only use 2 cards".to_string());
    }

    for &idx in indices {
        let rank = get_card_rank(&hand[idx]);
        let next_rank = Rank::from_repr(rank as u8 + 1).unwrap_or(Rank::Two);
        set_card_rank(&mut hand[idx], next_rank);
    }

    Ok(())
}

pub fn use_hanged_man(hand: &mut Vec<Card>, indices: &[usize]) -> Result<(), String> {
    if indices.is_empty() {
        return Err("No cards selected".to_string());
    }

    if indices.len() > 2 {
        return Err("You can only use 2 cards".to_string());
    }

    let mut sorted_indices = indices.to_vec();
    sorted_indices.sort_unstable();

    for idx in sorted_indices.into_iter().rev() {
        if idx < hand.len() {
            hand.remove(idx);
        }
    }

    Ok(())
}

pub fn use_death(hand: &mut [Card], indices: &[usize]) -> Result<(), String> {
    if indices.is_empty() {
        return Err("No cards selected".to_string());
    }

    if indices.len() != 2 {
        return Err("You must use 2 cards".to_string());
    }

    hand[indices[0]] = hand[indices[1]];

    Ok(())
}

pub fn use_temperance(deck: &mut [Card], indices: &[usize]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

pub fn use_judgement(deck: &mut [Card], indices: &[usize]) -> Result<(), String> {
    return Err("Not coded yet".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameState;
    use crate::card::core::create_test_card;
    use crate::card::operations::{get_card_enhancement, get_card_rank, get_card_suit};
    use crate::consumable::Consumable;

    fn dummy_game_state(deck: Vec<Card>) -> GameState {
        GameState {
            last_used: Consumable::Tarot(Tarot::Fool),
            tarots_used: 0,
            hand: deck.clone(),
            deck,
            hand_size: 8,
            jokers: vec![],
            joker_slots: 5,
            consumables: vec![],
            consumable_slots: 2,
            balance: 4,
        }
    }

    fn use_enhancement_tarot(
        tarot: Tarot,
        deck: Vec<Card>,
        indices: &[usize],
    ) -> Result<Vec<Card>, String> {
        let mut gs = dummy_game_state(deck);
        use_tarot(&mut gs, tarot, indices)?;
        Ok(gs.hand)
    }

    fn use_suit_tarot(
        tarot: Tarot,
        deck: Vec<Card>,
        indices: &[usize],
    ) -> Result<Vec<Card>, String> {
        let mut gs = dummy_game_state(deck);
        use_tarot(&mut gs, tarot, indices)?;
        Ok(gs.hand)
    }

    #[test]
    fn test_tarot_strength() {
        let mut deck = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
        ];

        let result = use_strength(&mut deck, &[0, 1]);
        assert!(result.is_ok());

        assert_eq!(get_card_rank(&deck[0]), Rank::Three);
        assert_eq!(get_card_rank(&deck[1]), Rank::Ace);
    }

    #[test]
    fn test_tarot_strength_ace() {
        let mut deck = vec![create_test_card(Rank::Ace, Suit::Spades)];
        let result = use_strength(&mut deck, &[0]);
        assert!(result.is_ok());
        assert_eq!(get_card_rank(&deck[0]), Rank::Two);
    }

    #[test]
    fn test_tarot_death() {
        let mut deck = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
        ];

        let result = use_death(&mut deck, &[0, 1]);
        assert!(result.is_ok());

        assert_eq!(get_card_rank(&deck[0]), Rank::King);
        assert_eq!(get_card_suit(&deck[0]), Suit::Hearts);
        assert_eq!(get_card_rank(&deck[1]), Rank::King);
        assert_eq!(get_card_suit(&deck[1]), Suit::Hearts);
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
            let deck = vec![create_test_card(Rank::Two, Suit::Spades); max_cards];
            let indices: Vec<usize> = (0..max_cards).collect();
            let deck = use_enhancement_tarot(tarot, deck, &indices).expect("Tarot failed");
            for card in &deck {
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
            let deck = vec![
                create_test_card(Rank::Two, Suit::Spades),
                create_test_card(Rank::Three, Suit::Hearts),
                create_test_card(Rank::Four, Suit::Clubs),
            ];
            let indices: Vec<usize> = (0..3).collect();
            let deck = use_suit_tarot(tarot, deck, &indices).expect("Tarot failed");
            for card in &deck {
                assert_eq!(get_card_suit(card), expected_suit);
            }
        }
    }
}
