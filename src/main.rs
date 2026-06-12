mod card;
use card::create_deck;
use card::parse_card_to_text;

fn main() {
    let cards = create_deck();

    println!("{:?}", cards);

    println!("{}", parse_card_to_text(&cards[34]));
}
