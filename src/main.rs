mod blinds;
mod card;
mod consumable;
mod levels;

use card::*;
use consumable::*;

pub struct GameState {
    last_used: Consumable,
    tarots_used: u16,
    deck: Vec<Card>,
    // starting_deck_size: u8,
    // skips_taken: u16,
    // hand_size: u8,
    // ecto_hand_size_reduction: u8, // starts at 1
}

fn main() {
    let mut game_state = GameState {
        last_used: Consumable::Tarot(Tarot::Fool),
        tarots_used: 0,
        deck: create_deck(),
    };

    let parsed_cards: Vec<String> = game_state.deck.iter().map(parse_card_to_text).collect();

    println!("Before use:\n{:?}", parsed_cards);

    println!(
        "\nSelected cards:\n{:?}",
        game_state.deck[0..2]
            .iter()
            .map(parse_card_to_text)
            .collect::<Vec<String>>()
    );

    // We use std::mem::take to temporarily extract the deck from game_state.
    // This allows us to pass &mut game_state AND a mutable slice of the deck to `use_tarot`
    // simultaneously without violating Rust's aliasing rules (borrow checker).
    // This requires exactly zero allocations and copies.
    let mut deck = std::mem::take(&mut game_state.deck);
    let result = use_tarot(&mut game_state, Tarot::Magician, &mut deck[0..2]);

    // Put the modified deck back into the game_state
    game_state.deck = deck;

    println!(
        "\nAfter use:\n{:?}",
        game_state
            .deck
            .iter()
            .map(parse_card_to_text)
            .collect::<Vec<String>>()
    );
    println!("\nResult: {:?}", result);
}
