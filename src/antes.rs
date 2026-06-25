use crate::blinds::Blind;
use crate::blinds::BossBlind;
use crate::stakes::Stake;

const BASE_SCORE_REQUIREMENT: [u32; 8] = [300, 800, 2_000, 5_000, 11_000, 20_000, 35_000, 50_000];

const GREEN_STAKE_SCORE_REQUIREMENT: [u32; 8] =
    [300, 900, 2_600, 8_000, 20_000, 36_000, 60_000, 100_000];

const PURPLE_STAKE_SCORE_REQUIREMENT: [u32; 8] =
    [300, 1_000, 3_200, 9_000, 25_000, 60_000, 110_000, 200_000];

pub fn get_required_score(
    ante: u8,
    blind: Blind,
    stake: Stake,
    boss_blind: Option<BossBlind>,
) -> f64 {
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
    let multiplier = match blind {
        Blind::Small => 1.0,
        Blind::Big => 1.5,
        Blind::Boss => 2.0,
    };
    let multiplier = match boss_blind {
        Some(blind) => match blind {
            BossBlind::TheWall => 4.0,
            BossBlind::TheNeedle => 1.0,
            _ => multiplier,
        },
        None => multiplier,
    };
    base_score as f64 * multiplier
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blinds::Blind::{Big, Boss, Small};
    use crate::blinds::BossBlind::{TheNeedle, TheWall};

    #[test]
    fn test_white_stake() {
        // Ante 1
        assert_eq!(get_required_score(0, Small, Stake::White, None), 300.0);
        assert_eq!(get_required_score(0, Big, Stake::White, None), 450.0);
        assert_eq!(get_required_score(0, Boss, Stake::White, None), 600.0);

        // Ante 2
        assert_eq!(get_required_score(1, Small, Stake::White, None), 800.0);
        assert_eq!(get_required_score(1, Big, Stake::White, None), 1_200.0);
        assert_eq!(get_required_score(1, Boss, Stake::White, None), 1_600.0);

        // Ante 3
        assert_eq!(get_required_score(2, Small, Stake::White, None), 2_000.0);
        assert_eq!(get_required_score(2, Big, Stake::White, None), 3_000.0);
        assert_eq!(get_required_score(2, Boss, Stake::White, None), 4_000.0);
    }

    #[test]
    fn test_green_stake() {
        // Ante 1
        assert_eq!(get_required_score(0, Small, Stake::Green, None), 300.0);
        assert_eq!(get_required_score(1, Small, Stake::Green, None), 900.0);

        // Ante 3 (scales faster)
        assert_eq!(get_required_score(2, Small, Stake::Green, None), 2_600.0);
        assert_eq!(get_required_score(2, Big, Stake::Green, None), 3_900.0);
        assert_eq!(get_required_score(2, Boss, Stake::Green, None), 5_200.0);

        // Ante 4
        assert_eq!(get_required_score(3, Small, Stake::Green, None), 8_000.0);
    }

    #[test]
    fn test_purple_stake() {
        // Ante 1
        assert_eq!(get_required_score(0, Small, Stake::Purple, None), 300.0);
        assert_eq!(get_required_score(1, Small, Stake::Purple, None), 1_000.0);

        // Ante 3 (scales even faster)
        assert_eq!(get_required_score(2, Small, Stake::Purple, None), 3_200.0);
        assert_eq!(get_required_score(2, Big, Stake::Purple, None), 4_800.0);
        assert_eq!(get_required_score(2, Boss, Stake::Purple, None), 6_400.0);

        // Ante 4
        assert_eq!(get_required_score(3, Small, Stake::Purple, None), 9_000.0);
    }

    #[test]
    fn test_max_ante_cap() {
        // Ante 8 (rounds ~25-27)
        assert_eq!(get_required_score(7, Small, Stake::White, None), 50_000.0);
        assert_eq!(get_required_score(7, Big, Stake::White, None), 75_000.0);
        assert_eq!(get_required_score(7, Boss, Stake::White, None), 100_000.0);
    }

    #[test]
    #[should_panic]
    fn test_beyond_max_ante_panics() {
        // Ante 10 (round 28+) should panic
        get_required_score(10, Small, Stake::White, None);
    }

    #[test]
    fn test_stake_mappings() {
        // Red maps to White
        assert_eq!(
            get_required_score(3, Small, Stake::Red, None),
            get_required_score(3, Small, Stake::White, None)
        );

        // Black and Blue map to Green
        assert_eq!(
            get_required_score(3, Small, Stake::Black, None),
            get_required_score(3, Small, Stake::Green, None)
        );
        assert_eq!(
            get_required_score(3, Small, Stake::Blue, None),
            get_required_score(3, Small, Stake::Green, None)
        );

        // Orange and Gold map to Purple
        assert_eq!(
            get_required_score(3, Small, Stake::Orange, None),
            get_required_score(3, Small, Stake::Purple, None)
        );
        assert_eq!(
            get_required_score(3, Small, Stake::Gold, None),
            get_required_score(3, Small, Stake::Purple, None)
        );
    }

    #[test]
    fn test_boss_blinds() {
        // Normal Boss Blind (Ante 1)
        assert_eq!(get_required_score(0, Boss, Stake::White, None), 600.0); // 300 * 2.0

        // The Wall (Ante 1)
        assert_eq!(
            get_required_score(0, Boss, Stake::White, Some(TheWall)),
            1_200.0
        ); // 300 * 4.0

        // The Needle (Ante 1)
        assert_eq!(
            get_required_score(0, Boss, Stake::White, Some(TheNeedle)),
            300.0
        ); // 300 * 1.0
    }
}
