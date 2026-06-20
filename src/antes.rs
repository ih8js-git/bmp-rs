use crate::blinds::BossBlind;
use crate::stakes::Stake;

const BASE_SCORE_REQUIREMENT: [u32; 9] =
    [100, 300, 800, 2_000, 5_000, 11_000, 20_000, 35_000, 50_000];

const GREEN_STAKE_SCORE_REQUIREMENT: [u32; 9] =
    [100, 300, 900, 2_600, 8_000, 20_000, 36_000, 60_000, 100_000];

const PURPLE_STAKE_SCORE_REQUIREMENT: [u32; 9] = [
    100, 300, 1_000, 3_200, 9_000, 25_000, 60_000, 110_000, 200_000,
];

pub fn get_required_chips(round: u8, stake: Stake, boss_blind: Option<BossBlind>) -> u32 {
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
    let multiplier = match boss_blind {
        Some(blind) => match blind {
            BossBlind::TheWall => 4.0,
            BossBlind::TheNeedle => 1.0,
            _ => multiplier,
        },
        None => multiplier,
    };
    (base_score as f32 * multiplier) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_stake() {
        // Ante 1
        assert_eq!(get_required_chips(1, Stake::White, None), 300);
        assert_eq!(get_required_chips(2, Stake::White, None), 450);
        assert_eq!(get_required_chips(3, Stake::White, None), 600);

        // Ante 2
        assert_eq!(get_required_chips(4, Stake::White, None), 800);
        assert_eq!(get_required_chips(5, Stake::White, None), 1_200);
        assert_eq!(get_required_chips(6, Stake::White, None), 1_600);

        // Ante 3
        assert_eq!(get_required_chips(7, Stake::White, None), 2_000);
        assert_eq!(get_required_chips(8, Stake::White, None), 3_000);
        assert_eq!(get_required_chips(9, Stake::White, None), 4_000);
    }

    #[test]
    fn test_green_stake() {
        // Ante 1 and 2 (same as white)
        assert_eq!(get_required_chips(1, Stake::Green, None), 300);
        assert_eq!(get_required_chips(4, Stake::Green, None), 900);

        // Ante 3 (scales faster)
        assert_eq!(get_required_chips(7, Stake::Green, None), 2_600);
        assert_eq!(get_required_chips(8, Stake::Green, None), 3_900);
        assert_eq!(get_required_chips(9, Stake::Green, None), 5_200);

        // Ante 4
        assert_eq!(get_required_chips(10, Stake::Green, None), 8_000);
    }

    #[test]
    fn test_purple_stake() {
        // Ante 1 and 2 (same as white)
        assert_eq!(get_required_chips(1, Stake::Purple, None), 300);
        assert_eq!(get_required_chips(4, Stake::Purple, None), 1_000);

        // Ante 3 (scales even faster)
        assert_eq!(get_required_chips(7, Stake::Purple, None), 3_200);
        assert_eq!(get_required_chips(8, Stake::Purple, None), 4_800);
        assert_eq!(get_required_chips(9, Stake::Purple, None), 6_400);

        // Ante 4
        assert_eq!(get_required_chips(10, Stake::Purple, None), 9_000);
    }

    #[test]
    fn test_max_ante_cap() {
        // Ante 8 is rounds 25-27
        assert_eq!(get_required_chips(25, Stake::White, None), 50_000); // 50_000 * 1.0
        assert_eq!(get_required_chips(26, Stake::White, None), 75_000); // 50_000 * 1.5
        assert_eq!(get_required_chips(27, Stake::White, None), 100_000); // 50_000 * 2.0
    }

    #[test]
    #[should_panic]
    fn test_beyond_max_ante_panics() {
        // Ante 10 (round 28) should panic
        get_required_chips(28, Stake::White, None);
    }

    #[test]
    fn test_stake_mappings() {
        // Red maps to White
        assert_eq!(
            get_required_chips(7, Stake::Red, None),
            get_required_chips(7, Stake::White, None)
        );

        // Black and Blue map to Green
        assert_eq!(
            get_required_chips(7, Stake::Black, None),
            get_required_chips(7, Stake::Green, None)
        );
        assert_eq!(
            get_required_chips(7, Stake::Blue, None),
            get_required_chips(7, Stake::Green, None)
        );

        // Orange and Gold map to Purple
        assert_eq!(
            get_required_chips(7, Stake::Orange, None),
            get_required_chips(7, Stake::Purple, None)
        );
        assert_eq!(
            get_required_chips(7, Stake::Gold, None),
            get_required_chips(7, Stake::Purple, None)
        );
    }

    #[test]
    fn test_boss_blinds() {
        // Normal Boss Blind (Round 3)
        assert_eq!(get_required_chips(3, Stake::White, None), 600); // 300 * 2.0

        // The Wall (Round 3)
        assert_eq!(
            get_required_chips(3, Stake::White, Some(BossBlind::TheWall)),
            1_200
        ); // 300 * 4.0

        // The Needle (Round 3)
        assert_eq!(
            get_required_chips(3, Stake::White, Some(BossBlind::TheNeedle)),
            300
        ); // 300 * 1.0
    }
}
