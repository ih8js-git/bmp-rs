mod card;
mod consumable;
mod levels;

use card::*;
use consumable::tarots::*;

fn main() {
    let cards = create_deck();

    let parsed_cards: Vec<String> = cards.iter().map(parse_card_to_text).collect();

    println!("{:?}", parsed_cards);
}
