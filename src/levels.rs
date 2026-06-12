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

fn hand_base_chips_and_mult(levels: u16, hand: u8) -> (u16, u16) {
    match hand {
        0 => (160 + (50 * levels), 16 + (3 * levels)),
        1 => (140 + (40 * levels), 14 + (4 * levels)),
        2 => (120 + (35 * levels), 12 + (3 * levels)),
        3 => (100 + (40 * levels), 8 + (4 * levels)),
        4 => (60 + (30 * levels), 7 + (3 * levels)),
        5 => (40 + (25 * levels), 4 + (2 * levels)),
        6 => (35 + (15 * levels), 4 + (2 * levels)),
        7 => (30 + (30 * levels), 4 + (3 * levels)),
        8 => (30 + (20 * levels), 3 + (2 * levels)),
        9 => (20 + (20 * levels), 2 + levels),
        10 => (10 + (15 * levels), 2 + levels),
        11 => (5 + (10 * levels), 1 + levels),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_0_all_hands() {
        assert_eq!(hand_base_chips_and_mult(0, 0), (160, 16)); // Flush Five
        assert_eq!(hand_base_chips_and_mult(0, 1), (140, 14)); // Flush House
        assert_eq!(hand_base_chips_and_mult(0, 2), (120, 12)); // Five of a Kind
        assert_eq!(hand_base_chips_and_mult(0, 3), (100, 8)); // Straight Flush
        assert_eq!(hand_base_chips_and_mult(0, 4), (60, 7)); // Four of a Kind
        assert_eq!(hand_base_chips_and_mult(0, 5), (40, 4)); // Full House
        assert_eq!(hand_base_chips_and_mult(0, 6), (35, 4)); // Flush
        assert_eq!(hand_base_chips_and_mult(0, 7), (30, 4)); // Straight
        assert_eq!(hand_base_chips_and_mult(0, 8), (30, 3)); // Three of a Kind
        assert_eq!(hand_base_chips_and_mult(0, 9), (20, 2)); // Two Pair
        assert_eq!(hand_base_chips_and_mult(0, 10), (10, 2)); // Pair
        assert_eq!(hand_base_chips_and_mult(0, 11), (5, 1)); // High Card
    }

    #[test]
    fn test_level_scaling() {
        // Flush Five (hand 0) at level 1: +50 chips, +3 mult
        assert_eq!(hand_base_chips_and_mult(1, 0), (210, 19));

        // High Card (hand 11) at level 5: +10 chips, +1 mult per level -> +50, +5
        assert_eq!(hand_base_chips_and_mult(5, 11), (55, 6));

        // Two Pair (hand 9) at level 10: +20 chips, +1 mult per level -> +200, +10
        assert_eq!(hand_base_chips_and_mult(10, 9), (220, 12));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_invalid_hand() {
        hand_base_chips_and_mult(0, 12);
    }
}
