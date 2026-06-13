use crate::card::{Card, Seal, set_card_seal};

enum Spectral {
    Familiar,
    Grim,
    Incantation,
    Tailisman,
    Aura,
    Wraith,
    Sigil,
    Oujia,
    Ectoplasm,
    Immolate,
    Ankh,
    DejaVu,
    Hex,
    Trance,
    Medium,
    Cryptid,
    Soul,
    BlackHole,
}

fn add_seal_to_card(card: Option<&mut Card>, seal: Seal) -> Result<(), String> {
    let c = match card {
        Some(c) => c,
        None => return Err("No card selected".to_string()),
    };

    set_card_seal(c, seal);
    Ok(())
}

fn use_seal_spectral(spectral: Spectral, card: Option<&mut Card>) -> Result<(), String> {
    match spectral {
        Spectral::Tailisman => add_seal_to_card(card, Seal::Gold),
        Spectral::DejaVu => add_seal_to_card(card, Seal::Red),
        Spectral::Trance => add_seal_to_card(card, Seal::Blue),
        Spectral::Medium => add_seal_to_card(card, Seal::Purple),
        _ => return Err("Not a valid spectral for this function".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Rank, Suit, create_test_card};
    use crate::card::operations::get_card_seal;

    #[test]
    fn test_use_seal_spectral_success() {
        let mut card = create_test_card(Rank::Two, Suit::Spades);
        assert!(use_seal_spectral(Spectral::Tailisman, Some(&mut card)).is_ok());
        assert_eq!(get_card_seal(&card), Seal::Gold);

        let mut card = create_test_card(Rank::Two, Suit::Spades);
        assert!(use_seal_spectral(Spectral::DejaVu, Some(&mut card)).is_ok());
        assert_eq!(get_card_seal(&card), Seal::Red);

        let mut card = create_test_card(Rank::Two, Suit::Spades);
        assert!(use_seal_spectral(Spectral::Trance, Some(&mut card)).is_ok());
        assert_eq!(get_card_seal(&card), Seal::Blue);

        let mut card = create_test_card(Rank::Two, Suit::Spades);
        assert!(use_seal_spectral(Spectral::Medium, Some(&mut card)).is_ok());
        assert_eq!(get_card_seal(&card), Seal::Purple);
    }

    #[test]
    fn test_use_seal_spectral_no_card() {
        let result = use_seal_spectral(Spectral::Tailisman, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No card selected".to_string());
    }

    #[test]
    fn test_use_seal_spectral_invalid_spectral() {
        let mut card = create_test_card(Rank::Two, Suit::Spades);
        let result = use_seal_spectral(Spectral::BlackHole, Some(&mut card));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Not a valid spectral for this function".to_string()
        );
    }
}
