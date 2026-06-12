/*
 * 0: Flush Five     | 160x16 | +50, +3
 * 1: Flush House    | 140x14 | +40, +4
 * 2: Five of a Kind | 120x12 | +35, +3
 * 3: Straight Flush | 100x8  | +40, +4
 * 4: Four of a Kind | 60x7   | +30, +3
 * 5: Full House     | 40x4   | +25, +2
 * 6: Flush          | 35x4   | +15, +2
 * 7: Straight       | 30x4   | +30, +3
 * 8: Three of a Kind| 30x3   | +20, +2
 * 9: Two Pair       | 20x2   | +20, +1
 * 10: Pair          | 10x2   | +15, +1
 * 11: High Card     | 5x1    | +10, +1
 */

enum Hand {
    FlushFive,
    FlushHouse,
    FiveOfAKind,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

fn hand_base_chips_and_mult(levels: u16, hand: Hand) -> (u16, u16) {
    match hand {
        Hand::FlushFive => (160 + (50 * levels), 16 + (3 * levels)),
        Hand::FlushHouse => (140 + (40 * levels), 14 + (4 * levels)),
        Hand::FiveOfAKind => (120 + (35 * levels), 12 + (3 * levels)),
        Hand::StraightFlush => (100 + (40 * levels), 8 + (4 * levels)),
        Hand::FourOfAKind => (60 + (30 * levels), 7 + (3 * levels)),
        Hand::FullHouse => (40 + (25 * levels), 4 + (2 * levels)),
        Hand::Flush => (35 + (15 * levels), 4 + (2 * levels)),
        Hand::Straight => (30 + (30 * levels), 4 + (3 * levels)),
        Hand::ThreeOfAKind => (30 + (20 * levels), 3 + (2 * levels)),
        Hand::TwoPair => (20 + (20 * levels), 2 + levels),
        Hand::Pair => (10 + (15 * levels), 2 + levels),
        Hand::HighCard => (5 + (10 * levels), 1 + levels),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_0_all_hands() {
        assert_eq!(hand_base_chips_and_mult(0, Hand::FlushFive), (160, 16));
        assert_eq!(hand_base_chips_and_mult(0, Hand::FlushHouse), (140, 14));
        assert_eq!(hand_base_chips_and_mult(0, Hand::FiveOfAKind), (120, 12));
        assert_eq!(hand_base_chips_and_mult(0, Hand::StraightFlush), (100, 8));
        assert_eq!(hand_base_chips_and_mult(0, Hand::FourOfAKind), (60, 7));
        assert_eq!(hand_base_chips_and_mult(0, Hand::FullHouse), (40, 4));
        assert_eq!(hand_base_chips_and_mult(0, Hand::Flush), (35, 4));
        assert_eq!(hand_base_chips_and_mult(0, Hand::Straight), (30, 4));
        assert_eq!(hand_base_chips_and_mult(0, Hand::ThreeOfAKind), (30, 3));
        assert_eq!(hand_base_chips_and_mult(0, Hand::TwoPair), (20, 2));
        assert_eq!(hand_base_chips_and_mult(0, Hand::Pair), (10, 2));
        assert_eq!(hand_base_chips_and_mult(0, Hand::HighCard), (5, 1));
    }

    #[test]
    fn test_level_scaling() {
        // Flush Five (hand 0) at level 1: +50 chips, +3 mult
        assert_eq!(hand_base_chips_and_mult(1, Hand::FlushFive), (210, 19));

        // High Card (hand 11) at level 5: +10 chips, +1 mult per level -> +50, +5
        assert_eq!(hand_base_chips_and_mult(5, Hand::HighCard), (55, 6));

        // Two Pair (hand 9) at level 10: +20 chips, +1 mult per level -> +200, +10
        assert_eq!(hand_base_chips_and_mult(10, Hand::TwoPair), (220, 12));
    }
}
