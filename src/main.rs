mod card;
mod consumables;
mod levels;

use card::*;
use consumables::tarots::*;

fn main() {
    let cards = create_deck();

    let parsed_cards: Vec<String> = cards.iter().map(parse_card_to_text).collect();

    println!("{:?}", parsed_cards);
}
