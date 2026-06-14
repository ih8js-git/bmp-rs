//mod blinds;
mod card;
mod consumable;
mod decks;
mod joker;
mod levels;
mod vouchers;

use card::*;
use consumable::*;
use decks::*;
use joker::*;
use vouchers::*;

pub struct GameState {
    last_used: Consumable,
    tarots_used: u16,
    deck: Vec<Card>,
    vouchers: u32,
    hand: Vec<Card>,
    hand_size: u8,
    jokers: Vec<Joker>,
    joker_slots: u8,
    consumables: Vec<Consumable>,
    consumable_slots: u8,
    balance: u32,
    hands: u8,
    discards: u8,
    current_round: u8,
    starting_deck_size: u8,
    skips_taken: u8,
    base_reroll_cost: u8,
    // ecto_hand_size_reduction: u8, // starts at 1
    // hands_played,
    // unused_discards,
    // how many each planet has been used
    // how many each hand has been played
}

fn main() {
    let mut game_state = create_game_state(Deck::Red);

    for _ in 0..game_state.hand_size {
        let card = game_state.deck.pop().unwrap();
        game_state.hand.push(card);
    }

    let parsed_cards: Vec<String> = game_state.hand.iter().map(parse_card_to_text).collect();

    println!("Before use:\n{:?}", parsed_cards);

    println!(
        "\nSelected cards:\n{:?}",
        game_state.hand[0..2]
            .iter()
            .map(parse_card_to_text)
            .collect::<Vec<String>>()
    );

    let result = use_tarot(&mut game_state, Tarot::Magician, &[0, 1]);

    println!(
        "\nAfter use:\n{:?}",
        game_state
            .hand
            .iter()
            .map(parse_card_to_text)
            .collect::<Vec<String>>()
    );
    println!("\nResult: {:?}", result);

    let result = use_tarot(&mut game_state, Tarot::Strength, &[0, 1]);

    println!(
        "\nAfter use:\n{:?}",
        game_state
            .hand
            .iter()
            .map(parse_card_to_text)
            .collect::<Vec<String>>()
    );
    println!("\nResult: {:?}", result);

    let result = use_tarot(&mut game_state, Tarot::Death, &[0, 1]);

    println!(
        "\nAfter use:\n{:?}",
        game_state
            .hand
            .iter()
            .map(parse_card_to_text)
            .collect::<Vec<String>>()
    );
    println!("\nResult: {:?}", result);

    let result = use_tarot(&mut game_state, Tarot::HangedMan, &[0, 1]);

    println!(
        "\nAfter use:\n{:?}",
        game_state
            .hand
            .iter()
            .map(parse_card_to_text)
            .collect::<Vec<String>>()
    );
    println!("\nResult: {:?}", result);
}
