use crate::card::operations::{get_card_enhancement, get_card_rank, get_card_suit};
use crate::card::{Card, Enhancement};
use crate::levels::Hand;

pub fn get_hand_type(hand: &[Card]) -> (Hand, Vec<usize>) {
    let mut ranks = [0u8; 13];
    let mut flush = false;

    for card in hand {
        ranks[get_card_rank(card) as usize] += 1;
    }

    fn is_flush(hand: &[Card]) -> bool {
        let mut target_suit = None;
        for card in hand {
            if get_card_enhancement(card) != Enhancement::Wild {
                let suit = get_card_suit(card);
                if let Some(t) = target_suit {
                    if suit != t {
                        return false; // Found conflicting suits
                    }
                } else {
                    target_suit = Some(suit); // First non-wild suit found
                }
            }
        }
        true // All non-wild cards match (or all cards are wild)
    }

    fn is_straight(hand: &[Card]) -> bool {
        let mut r: Vec<usize> = hand.iter().map(|c| get_card_rank(c) as usize).collect();
        r.sort_unstable(); // Sort ranks from lowest (Two) to highest (Ace)

        // Standard straight check (e.g. 5, 6, 7, 8, 9)
        if r.windows(2).all(|w| w[1] == w[0] + 1) {
            return true;
        }

        // Ace-low straight check (A, 2, 3, 4, 5)
        // Since Ace is 12 and Two is 0, a sorted Ace-low straight looks like: [0, 1, 2, 3, 12]
        if *r.last().unwrap() == 12 {
            let mut is_low_ace = true;
            for i in 0..(r.len() - 1) {
                if r[i] != i {
                    // Must be Two(0), Three(1), Four(2), etc.
                    is_low_ace = false;
                    break;
                }
            }
            if is_low_ace {
                return true;
            }
        }

        false
    }

    fn check_five_card_hands(
        hand: &[Card],
        ranks: &[u8; 13],
        flush: bool,
    ) -> Option<(Hand, Vec<usize>)> {
        if ranks.contains(&5) {
            if flush {
                return Some((Hand::FlushFive, (0..hand.len()).collect()));
            } else {
                return Some((Hand::FiveOfAKind, (0..hand.len()).collect()));
            }
        } else if is_straight(hand) {
            if flush {
                return Some((Hand::StraightFlush, (0..hand.len()).collect()));
            } else {
                return Some((Hand::Straight, (0..hand.len()).collect()));
            }
        } else if ranks.contains(&2) && ranks.contains(&3) {
            if flush {
                return Some((Hand::FlushHouse, (0..hand.len()).collect()));
            } else {
                return Some((Hand::FullHouse, (0..hand.len()).collect()));
            }
        } else if flush {
            return Some((Hand::Flush, (0..hand.len()).collect()));
        }
        None
    }

    fn check_four_card_hands(hand: &[Card], ranks: &[u8; 13]) -> Option<(Hand, Vec<usize>)> {
        if ranks.contains(&4) {
            // Find the rank that has 4 cards, and only return those indices
            let target_rank = ranks.iter().position(|&count| count == 4).unwrap();
            let mut indices = Vec::new();
            for (i, card) in hand.iter().enumerate() {
                if get_card_rank(card) as usize == target_rank {
                    indices.push(i);
                }
            }
            return Some((Hand::FourOfAKind, indices));
        } else if ranks.iter().filter(|&&count| count == 2).count() == 2 {
            let mut indices = Vec::new();
            for (i, card) in hand.iter().enumerate() {
                if ranks[get_card_rank(card) as usize] == 2 {
                    indices.push(i);
                }
            }
            return Some((Hand::TwoPair, indices));
        }
        None
    }

    fn check_three_card_hands(hand: &[Card], ranks: &[u8; 13]) -> Option<(Hand, Vec<usize>)> {
        if ranks.contains(&3) {
            // Find the rank that has 3 cards, and only return those indices
            let target_rank = ranks.iter().position(|&count| count == 3).unwrap();
            let mut indices = Vec::new();
            for (i, card) in hand.iter().enumerate() {
                if get_card_rank(card) as usize == target_rank {
                    indices.push(i);
                }
            }
            return Some((Hand::ThreeOfAKind, indices));
        }
        None
    }

    fn check_two_card_hands(hand: &[Card], ranks: &[u8; 13]) -> Option<(Hand, Vec<usize>)> {
        if ranks.contains(&2) {
            // Find the rank that has 2 cards, and only return those indices
            let target_rank = ranks.iter().position(|&count| count == 2).unwrap();
            let mut indices = Vec::new();
            for (i, card) in hand.iter().enumerate() {
                if get_card_rank(card) as usize == target_rank {
                    indices.push(i);
                }
            }
            return Some((Hand::Pair, indices));
        }
        None
    }

    if is_flush(hand) {
        flush = true;
    }

    if hand.len() == 5 {
        // If the hand doesn't have 5 cards, no point checking for five card hands.
        // Unless they have the Four Fingers joker.
        if let Some(h) = check_five_card_hands(hand, &ranks, flush) {
            return h;
        }
    }

    if hand.len() >= 4 {
        if let Some(h) = check_four_card_hands(hand, &ranks) {
            return h;
        }
    }

    if hand.len() >= 3 {
        if let Some(h) = check_three_card_hands(hand, &ranks) {
            return h;
        }
    }

    if hand.len() >= 2 {
        if let Some(h) = check_two_card_hands(hand, &ranks) {
            return h;
        }
    }

    // High Card: find the card with the highest rank
    let mut highest_index = 0;
    let mut highest_rank = 0;
    for (i, card) in hand.iter().enumerate() {
        let rank = get_card_rank(card) as usize;
        if rank >= highest_rank {
            highest_rank = rank;
            highest_index = i;
        }
    }
    return (Hand::HighCard, vec![highest_index]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::core::{Enhancement, Rank, Suit, create_test_card};
    use crate::card::operations::set_card_enhancement;

    fn get_wild(rank: Rank, suit: Suit) -> Card {
        let mut c = create_test_card(rank, suit);
        set_card_enhancement(&mut c, Enhancement::Wild);
        c
    }

    #[test]
    fn test_flush_five() {
        let hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Spades),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::FlushFive, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_five_of_a_kind() {
        let hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
            create_test_card(Rank::King, Suit::Clubs),
            create_test_card(Rank::King, Suit::Diamonds),
            create_test_card(Rank::King, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand),
            (Hand::FiveOfAKind, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_flush_house() {
        let hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::Queen, Suit::Spades),
            create_test_card(Rank::Queen, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand),
            (Hand::FlushHouse, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_straight_flush() {
        let hand = vec![
            create_test_card(Rank::Five, Suit::Spades),
            create_test_card(Rank::Six, Suit::Spades),
            create_test_card(Rank::Seven, Suit::Spades),
            create_test_card(Rank::Eight, Suit::Spades),
            create_test_card(Rank::Nine, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand),
            (Hand::StraightFlush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_ace_low_straight_flush() {
        // A, 2, 3, 4, 5
        let hand = vec![
            create_test_card(Rank::Ace, Suit::Hearts),
            create_test_card(Rank::Two, Suit::Hearts),
            create_test_card(Rank::Three, Suit::Hearts),
            create_test_card(Rank::Four, Suit::Hearts),
            create_test_card(Rank::Five, Suit::Hearts),
        ];
        assert_eq!(
            get_hand_type(&hand),
            (Hand::StraightFlush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_four_of_a_kind_ideal() {
        let hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
            create_test_card(Rank::King, Suit::Clubs),
            create_test_card(Rank::King, Suit::Diamonds),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::FourOfAKind, vec![0, 1, 2, 3]));
    }

    #[test]
    fn test_four_of_a_kind_less_ideal() {
        // 5 cards played, one kicker
        let hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::Queen, Suit::Hearts), // junk
            create_test_card(Rank::King, Suit::Hearts),
            create_test_card(Rank::King, Suit::Clubs),
            create_test_card(Rank::King, Suit::Diamonds),
        ];
        // The scoring cards are the Kings: indices 0, 2, 3, 4
        assert_eq!(get_hand_type(&hand), (Hand::FourOfAKind, vec![0, 2, 3, 4]));
    }

    #[test]
    fn test_full_house() {
        let hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::King, Suit::Hearts),
            create_test_card(Rank::King, Suit::Clubs),
            create_test_card(Rank::Queen, Suit::Spades),
            create_test_card(Rank::Queen, Suit::Hearts),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::FullHouse, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_flush_ideal() {
        let hand = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::Four, Suit::Spades),
            create_test_card(Rank::Six, Suit::Spades),
            create_test_card(Rank::Eight, Suit::Spades),
            create_test_card(Rank::Ten, Suit::Spades),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::Flush, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_flush_with_wilds() {
        let hand = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::Four, Suit::Spades),
            create_test_card(Rank::Six, Suit::Spades),
            get_wild(Rank::Eight, Suit::Hearts), // Wild card
            get_wild(Rank::Ten, Suit::Diamonds), // Wild card
        ];
        assert_eq!(get_hand_type(&hand), (Hand::Flush, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_straight_ideal() {
        let hand = vec![
            create_test_card(Rank::Seven, Suit::Spades),
            create_test_card(Rank::Eight, Suit::Hearts),
            create_test_card(Rank::Nine, Suit::Clubs),
            create_test_card(Rank::Ten, Suit::Spades),
            create_test_card(Rank::Jack, Suit::Hearts),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::Straight, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_ace_low_straight_mixed_suits() {
        let hand = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::Three, Suit::Hearts),
            create_test_card(Rank::Four, Suit::Clubs),
            create_test_card(Rank::Five, Suit::Spades),
            create_test_card(Rank::Ace, Suit::Hearts), // Ace is at end
        ];
        assert_eq!(get_hand_type(&hand), (Hand::Straight, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_three_of_a_kind_less_ideal() {
        let hand = vec![
            create_test_card(Rank::Three, Suit::Spades), // junk
            create_test_card(Rank::King, Suit::Hearts),
            create_test_card(Rank::King, Suit::Clubs),
            create_test_card(Rank::Four, Suit::Diamonds), // junk
            create_test_card(Rank::King, Suit::Spades),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::ThreeOfAKind, vec![1, 2, 4]));
    }

    #[test]
    fn test_two_pair_less_ideal() {
        let hand = vec![
            create_test_card(Rank::King, Suit::Spades),
            create_test_card(Rank::Queen, Suit::Hearts),
            create_test_card(Rank::Two, Suit::Clubs), // junk
            create_test_card(Rank::King, Suit::Clubs),
            create_test_card(Rank::Queen, Suit::Spades),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::TwoPair, vec![0, 1, 3, 4]));
    }

    #[test]
    fn test_pair_less_ideal() {
        let hand = vec![
            create_test_card(Rank::Three, Suit::Spades), // junk
            create_test_card(Rank::King, Suit::Hearts),
            create_test_card(Rank::Four, Suit::Clubs), // junk
            create_test_card(Rank::King, Suit::Clubs),
            create_test_card(Rank::Two, Suit::Spades), // junk
        ];
        assert_eq!(get_hand_type(&hand), (Hand::Pair, vec![1, 3]));
    }

    #[test]
    fn test_high_card_less_ideal() {
        let hand = vec![
            create_test_card(Rank::Two, Suit::Spades),
            create_test_card(Rank::Four, Suit::Hearts),
            create_test_card(Rank::King, Suit::Clubs), // High card
            create_test_card(Rank::Six, Suit::Diamonds),
            create_test_card(Rank::Eight, Suit::Spades),
        ];
        assert_eq!(get_hand_type(&hand), (Hand::HighCard, vec![2]));
    }
}
