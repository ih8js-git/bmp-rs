mod antes;
mod blinds;
mod card;
mod consumable;
mod decks;
mod game;
mod helper;
mod joker;
mod levels;
mod rng;
mod score;
mod stakes;
mod tables;
mod vouchers;

use decks::*;
use game::*;
use std::io;
use strum::IntoEnumIterator;
use vouchers::*;

fn main() {
    println!("Choose a deck:");
    for (index, deck) in Deck::iter().enumerate() {
        println!("{}: {:?}", index, deck);
    }
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let clean_input = input.trim();

    let deck = match clean_input {
        "0" => Deck::Red,
        "1" => Deck::Blue,
        "2" => Deck::Yellow,
        "3" => Deck::Green,
        "4" => Deck::Black,
        "5" => Deck::Magic,
        "6" => Deck::Nebula,
        "7" => Deck::Ghost,
        "8" => Deck::Abandoned,
        "9" => Deck::Checkered,
        "10" => Deck::Zodiac,
        "11" => Deck::Painted,
        "12" => Deck::Anaglyph,
        "13" => Deck::Plasma,
        "14" => Deck::Erratic,
        _ => panic!("Invalid deck"),
    };

    let mut gs = create_game_state(deck);

    println!("Choose to either Play the Small Blind, or Skip");
    println!("0: Play the Small Blind");
    println!("1: Skip");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let clean_input = input.trim();
    match clean_input {
        "0" => {
            println!("Playing the Small Blind");
        }
        "1" => {
            panic!("Skipping the Small Blind is not implemented yet");
        }
        _ => panic!("Invalid choice"),
    }

    gs.round += 1;
    println!("Round {}:", gs.round);
    println!(
        "Required chips: {}",
        antes::get_required_chips(gs.round, gs.stake, None)
    );
}
