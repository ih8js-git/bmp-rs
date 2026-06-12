mod card;
use card::Card;
use card::create_deck;

fn main() {
    let cards: Vec<Card> = create_deck();

    println!("{:?}", cards);
}
