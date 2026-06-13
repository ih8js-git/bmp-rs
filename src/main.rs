//mod blinds;
mod card;
mod consumable;
mod levels;

use card::*;
use consumable::*;

pub struct GameState {
    last_used: Consumable,
    tarots_used: u16,
    deck: Vec<Card>,
    hand: Vec<Card>, // starting_deck_size: u8,
                     // skips_taken: u16,
                     // hand_size: u8,
                     // ecto_hand_size_reduction: u8, // starts at 1
                     // hands_played,
                     // unused_discards,
                     // base_reroll_cost,
                     // current_round,
                     // how many each planet has been used
                     // how many each hand has been played
}

fn main() {
    let mut game_state = GameState {
        last_used: Consumable::Tarot(Tarot::Fool),
        tarots_used: 0,
        deck: create_deck(),
        hand: Vec::with_capacity(8),
    };

    for _ in 0..8 {
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
