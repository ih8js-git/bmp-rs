use crate::stakes::Stake;

const BASE_SCORE_REQUIREMENT: [u32; 9] =
    [100, 300, 800, 2_000, 5_000, 11_000, 20_000, 35_000, 50_000];

const GREEN_STAKE_SCORE_REQUIREMENT: [u32; 9] =
    [100, 300, 900, 2_600, 8_000, 20_000, 36_000, 60_000, 100_000];

const PURPLE_STAKE_SCORE_REQUIREMENT: [u32; 9] = [
    100, 300, 1_000, 3_200, 9_000, 25_000, 60_000, 110_000, 200_000,
];

pub fn get_required_chips(round: u8, stake: Stake) -> u32 {
    let ante = round.div_ceil(3);
    if ante > 9 {
        panic!("Antes beyond 9 are not supported yet");
    }
    let ante_idx = (ante as usize).min(8);
    let base_score = match stake {
        Stake::White => BASE_SCORE_REQUIREMENT[ante_idx],
        Stake::Red => BASE_SCORE_REQUIREMENT[ante_idx],
        Stake::Green => GREEN_STAKE_SCORE_REQUIREMENT[ante_idx],
        Stake::Black => GREEN_STAKE_SCORE_REQUIREMENT[ante_idx],
        Stake::Blue => GREEN_STAKE_SCORE_REQUIREMENT[ante_idx],
        Stake::Purple => PURPLE_STAKE_SCORE_REQUIREMENT[ante_idx],
        Stake::Orange => PURPLE_STAKE_SCORE_REQUIREMENT[ante_idx],
        Stake::Gold => PURPLE_STAKE_SCORE_REQUIREMENT[ante_idx],
    };
    let multiplier = match round % 3 {
        1 => 1.0,
        2 => 1.5,
        0 => 2.0,
        _ => unreachable!(),
    };
    (base_score as f32 * multiplier) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_stake() {
        // Ante 1
        assert_eq!(get_required_chips(1, Stake::White), 300);
        assert_eq!(get_required_chips(2, Stake::White), 450);
        assert_eq!(get_required_chips(3, Stake::White), 600);

        // Ante 2
        assert_eq!(get_required_chips(4, Stake::White), 800);
        assert_eq!(get_required_chips(5, Stake::White), 1_200);
        assert_eq!(get_required_chips(6, Stake::White), 1_600);

        // Ante 3
        assert_eq!(get_required_chips(7, Stake::White), 2_000);
        assert_eq!(get_required_chips(8, Stake::White), 3_000);
        assert_eq!(get_required_chips(9, Stake::White), 4_000);
    }

    #[test]
    fn test_green_stake() {
        // Ante 1 and 2 (same as white)
        assert_eq!(get_required_chips(1, Stake::Green), 300);
        assert_eq!(get_required_chips(4, Stake::Green), 900);

        // Ante 3 (scales faster)
        assert_eq!(get_required_chips(7, Stake::Green), 2_600);
        assert_eq!(get_required_chips(8, Stake::Green), 3_900);
        assert_eq!(get_required_chips(9, Stake::Green), 5_200);

        // Ante 4
        assert_eq!(get_required_chips(10, Stake::Green), 8_000);
    }

    #[test]
    fn test_purple_stake() {
        // Ante 1 and 2 (same as white)
        assert_eq!(get_required_chips(1, Stake::Purple), 300);
        assert_eq!(get_required_chips(4, Stake::Purple), 1_000);

        // Ante 3 (scales even faster)
        assert_eq!(get_required_chips(7, Stake::Purple), 3_200);
        assert_eq!(get_required_chips(8, Stake::Purple), 4_800);
        assert_eq!(get_required_chips(9, Stake::Purple), 6_400);

        // Ante 4
        assert_eq!(get_required_chips(10, Stake::Purple), 9_000);
    }

    #[test]
    fn test_max_ante_cap() {
        // Ante 8 is rounds 25-27
        assert_eq!(get_required_chips(25, Stake::White), 50_000); // 50_000 * 1.0
        assert_eq!(get_required_chips(26, Stake::White), 75_000); // 50_000 * 1.5
        assert_eq!(get_required_chips(27, Stake::White), 100_000); // 50_000 * 2.0
    }

    #[test]
    #[should_panic]
    fn test_beyond_max_ante_panics() {
        // Ante 10 (round 28) should panic
        get_required_chips(28, Stake::White);
    }

    #[test]
    fn test_stake_mappings() {
        // Red maps to White
        assert_eq!(
            get_required_chips(7, Stake::Red),
            get_required_chips(7, Stake::White)
        );

        // Black and Blue map to Green
        assert_eq!(
            get_required_chips(7, Stake::Black),
            get_required_chips(7, Stake::Green)
        );
        assert_eq!(
            get_required_chips(7, Stake::Blue),
            get_required_chips(7, Stake::Green)
        );

        // Orange and Gold map to Purple
        assert_eq!(
            get_required_chips(7, Stake::Orange),
            get_required_chips(7, Stake::Purple)
        );
        assert_eq!(
            get_required_chips(7, Stake::Gold),
            get_required_chips(7, Stake::Purple)
        );
    }
}
