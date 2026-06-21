//mod blinds;
mod card;
mod consumable;
mod decks;
mod game;
mod joker;
mod levels;
mod rng;
mod score;
mod vouchers;

use card::*;
use consumable::*;
use decks::*;
use game::*;
use vouchers::*;

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
