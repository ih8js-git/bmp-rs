use crate::blinds::BossBlind;
use crate::game::Blind;
use crate::stakes::Stake;

const BASE_SCORE_REQUIREMENT: [u32; 9] =
    [100, 300, 800, 2_000, 5_000, 11_000, 20_000, 35_000, 50_000];

const GREEN_STAKE_SCORE_REQUIREMENT: [u32; 9] =
    [100, 300, 900, 2_600, 8_000, 20_000, 36_000, 60_000, 100_000];

const PURPLE_STAKE_SCORE_REQUIREMENT: [u32; 9] = [
    100, 300, 1_000, 3_200, 9_000, 25_000, 60_000, 110_000, 200_000,
];

pub fn get_required_chips(
    ante: u8,
    blind: Blind,
    stake: Stake,
    boss_blind: Option<BossBlind>,
) -> u32 {
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
        Blind::SmallBlind => 1.0,
        Blind::BigBlind => 1.5,
        Blind::BossBlind => 2.0,
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
    use crate::blinds::BossBlind::{TheNeedle, TheWall};
    use crate::game::Blind::{BigBlind, BossBlind, SmallBlind};

    #[test]
    fn test_white_stake() {
        // Ante 1
        assert_eq!(get_required_chips(1, SmallBlind, Stake::White, None), 300);
        assert_eq!(get_required_chips(1, BigBlind, Stake::White, None), 450);
        assert_eq!(get_required_chips(1, BossBlind, Stake::White, None), 600);

        // Ante 2
        assert_eq!(get_required_chips(2, SmallBlind, Stake::White, None), 800);
        assert_eq!(get_required_chips(2, BigBlind, Stake::White, None), 1_200);
        assert_eq!(get_required_chips(2, BossBlind, Stake::White, None), 1_600);

        // Ante 3
        assert_eq!(get_required_chips(3, SmallBlind, Stake::White, None), 2_000);
        assert_eq!(get_required_chips(3, BigBlind, Stake::White, None), 3_000);
        assert_eq!(get_required_chips(3, BossBlind, Stake::White, None), 4_000);
    }

    #[test]
    fn test_green_stake() {
        // Ante 1
        assert_eq!(get_required_chips(1, SmallBlind, Stake::Green, None), 300);
        assert_eq!(get_required_chips(2, SmallBlind, Stake::Green, None), 900);

        // Ante 3 (scales faster)
        assert_eq!(get_required_chips(3, SmallBlind, Stake::Green, None), 2_600);
        assert_eq!(get_required_chips(3, BigBlind, Stake::Green, None), 3_900);
        assert_eq!(get_required_chips(3, BossBlind, Stake::Green, None), 5_200);

        // Ante 4
        assert_eq!(get_required_chips(4, SmallBlind, Stake::Green, None), 8_000);
    }

    #[test]
    fn test_purple_stake() {
        // Ante 1
        assert_eq!(get_required_chips(1, SmallBlind, Stake::Purple, None), 300);
        assert_eq!(
            get_required_chips(2, SmallBlind, Stake::Purple, None),
            1_000
        );

        // Ante 3 (scales even faster)
        assert_eq!(
            get_required_chips(3, SmallBlind, Stake::Purple, None),
            3_200
        );
        assert_eq!(get_required_chips(3, BigBlind, Stake::Purple, None), 4_800);
        assert_eq!(get_required_chips(3, BossBlind, Stake::Purple, None), 6_400);

        // Ante 4
        assert_eq!(
            get_required_chips(4, SmallBlind, Stake::Purple, None),
            9_000
        );
    }

    #[test]
    fn test_max_ante_cap() {
        // Ante 8 (rounds ~25-27)
        assert_eq!(
            get_required_chips(8, SmallBlind, Stake::White, None),
            50_000
        );
        assert_eq!(get_required_chips(8, BigBlind, Stake::White, None), 75_000);
        assert_eq!(
            get_required_chips(8, BossBlind, Stake::White, None),
            100_000
        );
    }

    #[test]
    #[should_panic]
    fn test_beyond_max_ante_panics() {
        // Ante 10 (round 28+) should panic
        get_required_chips(10, SmallBlind, Stake::White, None);
    }

    #[test]
    fn test_stake_mappings() {
        // Red maps to White
        assert_eq!(
            get_required_chips(3, SmallBlind, Stake::Red, None),
            get_required_chips(3, SmallBlind, Stake::White, None)
        );

        // Black and Blue map to Green
        assert_eq!(
            get_required_chips(3, SmallBlind, Stake::Black, None),
            get_required_chips(3, SmallBlind, Stake::Green, None)
        );
        assert_eq!(
            get_required_chips(3, SmallBlind, Stake::Blue, None),
            get_required_chips(3, SmallBlind, Stake::Green, None)
        );

        // Orange and Gold map to Purple
        assert_eq!(
            get_required_chips(3, SmallBlind, Stake::Orange, None),
            get_required_chips(3, SmallBlind, Stake::Purple, None)
        );
        assert_eq!(
            get_required_chips(3, SmallBlind, Stake::Gold, None),
            get_required_chips(3, SmallBlind, Stake::Purple, None)
        );
    }

    #[test]
    fn test_boss_blinds() {
        // Normal Boss Blind (Ante 1)
        assert_eq!(get_required_chips(1, BossBlind, Stake::White, None), 600); // 300 * 2.0

        // The Wall (Ante 1)
        assert_eq!(
            get_required_chips(1, BossBlind, Stake::White, Some(TheWall)),
            1_200
        ); // 300 * 4.0

        // The Needle (Ante 1)
        assert_eq!(
            get_required_chips(1, BossBlind, Stake::White, Some(TheNeedle)),
            300
        ); // 300 * 1.0
    }
}
