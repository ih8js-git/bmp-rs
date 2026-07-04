use crate::GameState;
use crate::card::operations::{get_card_enhancement, get_card_rank, get_card_suit};
use crate::card::{Card, Enhancement};
use crate::joker::Joker;
use crate::joker::fn_arrays::retrigger::RETRIGGER_FNS;
use crate::levels::Hand;

use crate::score::score_jokers::score_jokers;
use crate::score::score_played_cards::score_played_cards;

pub fn get_score(game_state: &mut GameState, cards_played: &[Card]) -> f64 {
    // 1. Score PreHand Stuff i.e. DNA, Midas Mask, etc.

    // 2. Determine the highest-ranking 5 card hand possible
    // TODO: Return more infromation about the hand, so that jokers, don't have to do repetitive checks
    let (hand_type, scoring_indices) = get_hand_type(cards_played, &game_state.jokers);

    // 3. Score the cards played i.e. lusty, dusk, etc.
    let current_level = game_state.planet_levels[hand_type as usize] as u16;

    let [mut chips, mut mult] = score_played_cards(
        cards_played,
        scoring_indices,
        hand_type,
        current_level,
        &game_state.jokers,
    );

    // 4. Score the cards held in hand i.e. steel, baron, etc.

    // 5. Score the jokers, i.e. Jolly Joker, etc.
    score_jokers(&game_state.jokers, hand_type, &mut chips, &mut mult);

    chips * mult
}

use crate::joker::JokerState;

fn get_flush_indices(
    hand: &[Card],
    jokers: &[JokerState],
    required_len: usize,
) -> Option<Vec<usize>> {
    let smeared = jokers.iter().any(|j| j.id() == Joker::SmearedJoker as u8);
    let mut suit_counts = [0; 4];
    let mut wild_indices = Vec::new();
    let mut suit_indices: [Vec<usize>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for (i, card) in hand.iter().enumerate() {
        if get_card_enhancement(card) == Enhancement::Wild {
            wild_indices.push(i);
        } else {
            let mut suit = get_card_suit(card);
            if smeared {
                if suit == crate::card::core::Suit::Diamonds {
                    suit = crate::card::core::Suit::Hearts;
                } else if suit == crate::card::core::Suit::Clubs {
                    suit = crate::card::core::Suit::Spades;
                }
            }
            suit_counts[suit as usize] += 1;
            suit_indices[suit as usize].push(i);
        }
    }

    let mut best_suit = None;
    let mut max_cards = 0;

    for i in 0..4 {
        let total = suit_counts[i] + wild_indices.len();
        if total >= required_len && total > max_cards {
            max_cards = total;
            best_suit = Some(i);
        }
    }

    if let Some(suit) = best_suit {
        let mut indices = Vec::new();
        for i in 0..hand.len() {
            if wild_indices.contains(&i) || suit_indices[suit].contains(&i) {
                indices.push(i);
            }
        }
        return Some(indices);
    }
    None
}

fn get_straight_indices(
    ranks: &[u8; 13],
    hand: &[Card],
    required_len: usize,
    jokers: &[JokerState],
) -> Option<Vec<usize>> {
    let shortcut = jokers.iter().any(|j| j.id() == Joker::Shortcut as u8);
    let max_gap = if shortcut { 2 } else { 1 };

    let mut present_ranks = Vec::new();
    if ranks[12] > 0 {
        present_ranks.push(-1_i32);
    }
    for i in 0..13 {
        if ranks[i] > 0 {
            present_ranks.push(i as i32);
        }
    }

    let mut best_straight_ranks = Vec::new();
    let mut current_straight = Vec::new();

    for i in 0..present_ranks.len() {
        if current_straight.is_empty() {
            current_straight.push(present_ranks[i]);
        } else {
            let last = *current_straight.last().unwrap();
            let diff = present_ranks[i] - last;
            if diff > 0 && diff <= max_gap {
                current_straight.push(present_ranks[i]);
            } else {
                if current_straight.len() >= required_len {
                    if current_straight.len() >= best_straight_ranks.len() {
                        best_straight_ranks = current_straight.clone();
                    }
                }
                current_straight.clear();
                current_straight.push(present_ranks[i]);
            }
        }
    }

    if current_straight.len() >= required_len {
        if current_straight.len() >= best_straight_ranks.len() {
            best_straight_ranks = current_straight.clone();
        }
    }

    if !best_straight_ranks.is_empty() {
        let mut indices = Vec::new();
        let take_count = 5.min(best_straight_ranks.len());
        let mut needed_ranks: Vec<usize> = best_straight_ranks
            .iter()
            .rev()
            .take(take_count)
            .map(|&r| if r == -1 { 12 } else { r as usize })
            .collect();

        for (idx, card) in hand.iter().enumerate() {
            let actual_rank = get_card_rank(card) as usize;
            if let Some(pos) = needed_ranks.iter().position(|&r| r == actual_rank) {
                indices.push(idx);
                needed_ranks.remove(pos); // Take only one card per needed rank
            }
        }
        return Some(indices);
    }

    None
}

fn check_five_card_hands(
    hand: &[Card],
    ranks: &[u8; 13],
    flush_indices: Option<Vec<usize>>,
    straight_indices: Option<Vec<usize>>,
) -> Option<(Hand, Vec<usize>)> {
    if ranks.contains(&5) {
        if flush_indices.is_some() {
            return Some((Hand::FlushFive, (0..hand.len()).collect()));
        } else {
            return Some((Hand::FiveOfAKind, (0..hand.len()).collect()));
        }
    } else if let Some(s_ind) = straight_indices {
        if let Some(f_ind) = flush_indices {
            let mut all_ind = Vec::new();
            for i in 0..hand.len() {
                if s_ind.contains(&i) || f_ind.contains(&i) {
                    all_ind.push(i);
                }
            }
            return Some((Hand::StraightFlush, all_ind));
        } else {
            return Some((Hand::Straight, s_ind));
        }
    } else if ranks.contains(&2) && ranks.contains(&3) {
        if flush_indices.is_some() {
            return Some((Hand::FlushHouse, (0..hand.len()).collect()));
        } else {
            return Some((Hand::FullHouse, (0..hand.len()).collect()));
        }
    } else if let Some(f_ind) = flush_indices {
        return Some((Hand::Flush, f_ind));
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

pub fn get_hand_type(hand: &[Card], jokers: &[JokerState]) -> (Hand, Vec<usize>) {
    let mut ranks = [0u8; 13];

    for card in hand {
        ranks[get_card_rank(card) as usize] += 1;
    }

    let four_fingers = jokers.iter().any(|j| j.id() == Joker::FourFingers as u8);
    let required_five_card_len = if four_fingers { 4 } else { 5 };

    let flush_indices = get_flush_indices(hand, jokers, required_five_card_len);
    let straight_indices = get_straight_indices(&ranks, hand, required_five_card_len, jokers);

    if hand.len() >= required_five_card_len {
        // If the hand doesn't have 5 cards, no point checking for five card hands.
        // Unless they have the Four Fingers joker.
        if let Some(h) = check_five_card_hands(hand, &ranks, flush_indices, straight_indices) {
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

pub fn count_retrigger_jokers(card: &Card, jokers: &[JokerState]) -> usize {
    let mut retriggers = 0;

    for joker in jokers {
        let id = joker.id() as usize;
        let retrigger_fn = RETRIGGER_FNS[id];
        // We have to pass both jokers, and joker, because
        // Sock needs to know if Pareidolia is active, as an example
        retriggers += retrigger_fn(card, jokers, joker);
    }

    retriggers
}

#[cfg(test)]
mod tests {
    use crate::joker::JokerState;
    fn get_test_jokers(jokers: &[Joker]) -> Vec<JokerState> {
        jokers
            .iter()
            .map(|j| {
                let mut s = JokerState::new();
                s.set_id(*j as u8);
                s
            })
            .collect()
    }
    use super::*;
    use crate::card::core::{Enhancement, Rank, Suit, create_card};
    use crate::card::operations::set_card_enhancement;

    fn get_wild(rank: Rank, suit: Suit) -> Card {
        let mut c = create_card(rank, suit);
        set_card_enhancement(&mut c, Enhancement::Wild);
        c
    }

    #[test]
    fn test_flush_five() {
        let hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::FlushFive, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_five_of_a_kind() {
        let hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Hearts),
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::King, Suit::Diamonds),
            create_card(Rank::King, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::FiveOfAKind, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_flush_house() {
        let hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::Queen, Suit::Spades),
            create_card(Rank::Queen, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::FlushHouse, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_straight_flush() {
        let hand = vec![
            create_card(Rank::Five, Suit::Spades),
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Seven, Suit::Spades),
            create_card(Rank::Eight, Suit::Spades),
            create_card(Rank::Nine, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::StraightFlush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_ace_low_straight_flush() {
        // A, 2, 3, 4, 5
        let hand = vec![
            create_card(Rank::Ace, Suit::Hearts),
            create_card(Rank::Two, Suit::Hearts),
            create_card(Rank::Three, Suit::Hearts),
            create_card(Rank::Four, Suit::Hearts),
            create_card(Rank::Five, Suit::Hearts),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::StraightFlush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_four_of_a_kind_ideal() {
        let hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Hearts),
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::King, Suit::Diamonds),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::FourOfAKind, vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn test_four_of_a_kind_less_ideal() {
        // 5 cards played, one kicker
        let hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::Queen, Suit::Hearts), // junk
            create_card(Rank::King, Suit::Hearts),
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::King, Suit::Diamonds),
        ];
        // The scoring cards are the Kings: indices 0, 2, 3, 4
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::FourOfAKind, vec![0, 2, 3, 4])
        );
    }

    #[test]
    fn test_full_house() {
        let hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Hearts),
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::Queen, Suit::Spades),
            create_card(Rank::Queen, Suit::Hearts),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::FullHouse, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_flush_ideal() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Spades),
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Eight, Suit::Spades),
            create_card(Rank::Ten, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::Flush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_flush_with_wilds() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Spades),
            create_card(Rank::Six, Suit::Spades),
            get_wild(Rank::Eight, Suit::Hearts), // Wild card
            get_wild(Rank::Ten, Suit::Diamonds), // Wild card
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::Flush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_straight_ideal() {
        let hand = vec![
            create_card(Rank::Seven, Suit::Spades),
            create_card(Rank::Eight, Suit::Hearts),
            create_card(Rank::Nine, Suit::Clubs),
            create_card(Rank::Ten, Suit::Spades),
            create_card(Rank::Jack, Suit::Hearts),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::Straight, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_ace_low_straight_mixed_suits() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Three, Suit::Hearts),
            create_card(Rank::Four, Suit::Clubs),
            create_card(Rank::Five, Suit::Spades),
            create_card(Rank::Ace, Suit::Hearts), // Ace is at end
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::Straight, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_three_of_a_kind_less_ideal() {
        let hand = vec![
            create_card(Rank::Three, Suit::Spades), // junk
            create_card(Rank::King, Suit::Hearts),
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::Four, Suit::Diamonds), // junk
            create_card(Rank::King, Suit::Spades),
        ];
        assert_eq!(
            get_hand_type(&hand, &[]),
            (Hand::ThreeOfAKind, vec![1, 2, 4])
        );
    }

    #[test]
    fn test_two_pair_less_ideal() {
        let hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::Queen, Suit::Hearts),
            create_card(Rank::Two, Suit::Clubs), // junk
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::Queen, Suit::Spades),
        ];
        assert_eq!(get_hand_type(&hand, &[]), (Hand::TwoPair, vec![0, 1, 3, 4]));
    }

    #[test]
    fn test_pair_less_ideal() {
        let hand = vec![
            create_card(Rank::Three, Suit::Spades), // junk
            create_card(Rank::King, Suit::Hearts),
            create_card(Rank::Four, Suit::Clubs), // junk
            create_card(Rank::King, Suit::Clubs),
            create_card(Rank::Two, Suit::Spades), // junk
        ];
        assert_eq!(get_hand_type(&hand, &[]), (Hand::Pair, vec![1, 3]));
    }

    #[test]
    fn test_high_card_less_ideal() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Hearts),
            create_card(Rank::King, Suit::Clubs), // High card
            create_card(Rank::Six, Suit::Diamonds),
            create_card(Rank::Eight, Suit::Spades),
        ];
        assert_eq!(get_hand_type(&hand, &[]), (Hand::HighCard, vec![2]));
    }

    #[test]
    fn test_smeared_joker_flush() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Spades),
            create_card(Rank::Six, Suit::Clubs),
            create_card(Rank::Eight, Suit::Spades),
            create_card(Rank::Ten, Suit::Clubs),
        ];
        // Without smeared joker it's a high card
        assert_eq!(get_hand_type(&hand, &[]), (Hand::HighCard, vec![4]));

        // With smeared joker it's a flush
        let jokers = get_test_jokers(&[Joker::SmearedJoker]);
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::Flush, vec![0, 1, 2, 3, 4])
        );

        let hand_red = vec![
            create_card(Rank::Two, Suit::Hearts),
            create_card(Rank::Four, Suit::Diamonds),
            create_card(Rank::Six, Suit::Hearts),
            create_card(Rank::Eight, Suit::Diamonds),
            create_card(Rank::Ten, Suit::Hearts),
        ];
        // Without smeared joker it's a high card
        assert_eq!(get_hand_type(&hand_red, &[]), (Hand::HighCard, vec![4]));

        // With smeared joker it's a flush
        assert_eq!(
            get_hand_type(&hand_red, &jokers),
            (Hand::Flush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_four_fingers() {
        // 4-card flush
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Spades),
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Eight, Suit::Spades),
            create_card(Rank::Ten, Suit::Hearts),
        ];

        let jokers = get_test_jokers(&[Joker::FourFingers]);
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::Flush, vec![0, 1, 2, 3])
        );

        // 4-card straight
        let hand2 = vec![
            create_card(Rank::Five, Suit::Spades),
            create_card(Rank::Six, Suit::Hearts),
            create_card(Rank::Seven, Suit::Spades),
            create_card(Rank::Eight, Suit::Clubs),
            create_card(Rank::King, Suit::Hearts),
        ];

        assert_eq!(
            get_hand_type(&hand2, &jokers),
            (Hand::Straight, vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn test_four_fingers_smeared() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Clubs), // Smeared makes this Spades
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Eight, Suit::Clubs), // Smeared makes this Spades
            create_card(Rank::Ten, Suit::Hearts),
        ];

        let jokers = get_test_jokers(&[Joker::FourFingers, Joker::SmearedJoker]);
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::Flush, vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn test_four_fingers_wild() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Spades),
            get_wild(Rank::Six, Suit::Hearts), // Wild counts as Spades
            get_wild(Rank::Eight, Suit::Diamonds), // Wild counts as Spades
            create_card(Rank::Ten, Suit::Hearts),
        ];

        let jokers = get_test_jokers(&[Joker::FourFingers]);
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::Flush, vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn test_four_fingers_straight_flush() {
        // 4-card straight flush
        let hand = vec![
            create_card(Rank::Five, Suit::Spades),
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Seven, Suit::Spades),
            create_card(Rank::Eight, Suit::Spades),
            create_card(Rank::King, Suit::Hearts), // Non-contributing card
        ];

        let jokers = get_test_jokers(&[Joker::FourFingers]);
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::StraightFlush, vec![0, 1, 2, 3])
        );

        // 4-card straight flush with overlapping straight/flush
        let hand_mixed = vec![
            create_card(Rank::Five, Suit::Spades),
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Seven, Suit::Spades),
            create_card(Rank::Two, Suit::Spades), // Contributes to flush
            create_card(Rank::Eight, Suit::Hearts), // Contributes to straight
        ];

        // This makes a 4-card straight (5, 6, 7, 8) and a 4-card flush (Spades)
        // Balatro evaluates this as a Straight Flush with 5 scoring cards.
        assert_eq!(
            get_hand_type(&hand_mixed, &jokers),
            (Hand::StraightFlush, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_shortcut_straight() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Hearts),
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Eight, Suit::Clubs),
            create_card(Rank::Ten, Suit::Hearts),
        ];

        let jokers = get_test_jokers(&[Joker::Shortcut]);
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::Straight, vec![0, 1, 2, 3, 4])
        );

        // Gap of 1 and 2
        let hand2 = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Three, Suit::Hearts),
            create_card(Rank::Five, Suit::Spades),
            create_card(Rank::Six, Suit::Clubs),
            create_card(Rank::Eight, Suit::Hearts),
        ];

        assert_eq!(
            get_hand_type(&hand2, &jokers),
            (Hand::Straight, vec![0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn test_shortcut_four_fingers() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Hearts),
            create_card(Rank::Six, Suit::Spades),
            create_card(Rank::Eight, Suit::Clubs),
            create_card(Rank::King, Suit::Hearts), // Non-contributing
        ];

        let jokers = get_test_jokers(&[Joker::Shortcut, Joker::FourFingers]);
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::Straight, vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn test_shortcut_four_fingers_smeared_wild_straight_flush() {
        let hand = vec![
            create_card(Rank::Two, Suit::Spades),
            create_card(Rank::Four, Suit::Clubs), // Smeared -> Spades
            get_wild(Rank::Six, Suit::Hearts),    // Wild -> Spades
            create_card(Rank::Eight, Suit::Spades),
            create_card(Rank::King, Suit::Diamonds), // Non-contributing
        ];

        let jokers = get_test_jokers(&[Joker::Shortcut, Joker::FourFingers, Joker::SmearedJoker]);

        // This evaluates to a 4-card Straight (2, 4, 6, 8 via Shortcut & Four Fingers)
        // and a 4-card Flush (Spades via Smeared & Wild & Four Fingers)
        assert_eq!(
            get_hand_type(&hand, &jokers),
            (Hand::StraightFlush, vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn test_sock_and_buskin_retrigger_flush_five() {
        use crate::decks::Deck;
        use crate::gamestate::create_game_state;

        let mut state = create_game_state(Deck::Red);
        let mut joker_state = JokerState::new();
        joker_state.set_id(Joker::SockAndBuskin as u8);
        state.jokers.push(joker_state);

        let mut hand = vec![
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
            create_card(Rank::King, Suit::Spades),
        ];

        // Hand: Flush Five Level 0: 160 base chips, 16 base mult.
        // 5 Kings, each has 10 chips.
        // Sock and buskin retriggers each face card (King) 1 time.
        // Total triggers per card = 2. Total cards = 5. Total triggers = 10.
        // Chips from cards: 10 * 10 = 100 chips.
        // Total chips: 160 + 100 = 260.
        // Total mult: 16.
        // Score = 260 * 16 = 4160.0.

        let result = get_score(&mut state, &mut hand);
        assert_eq!(result, 4160.0);
    }
}
