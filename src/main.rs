mod card;
mod consumables;
mod levels;

use card::*;
use consumables::tarots::*;

fn main() {
    let mut cards = create_deck();

    println!("{:?}", cards);

    println!("{}", parse_card_to_text(&cards[34]));
}
