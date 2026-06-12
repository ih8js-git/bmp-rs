mod card;

use card::*;

fn main() {
    let cards = create_deck();

    println!("{:?}", cards);

    println!("{}", parse_card_to_text(&cards[34]));
}
