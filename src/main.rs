mod card;
mod levels;
mod tarots;

use card::*;
use tarots::*;

fn main() {
    let mut cards = create_deck();

    println!("{:?}", cards);

    println!("{}", parse_card_to_text(&cards[34]));
}
