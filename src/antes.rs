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
    let base_score = match stake {
        Stake::White => BASE_SCORE_REQUIREMENT[ante as usize],
        Stake::Red => BASE_SCORE_REQUIREMENT[ante as usize],
        Stake::Green => GREEN_STAKE_SCORE_REQUIREMENT[ante as usize],
        Stake::Black => GREEN_STAKE_SCORE_REQUIREMENT[ante as usize],
        Stake::Blue => GREEN_STAKE_SCORE_REQUIREMENT[ante as usize],
        Stake::Purple => PURPLE_STAKE_SCORE_REQUIREMENT[ante as usize],
        Stake::Orange => PURPLE_STAKE_SCORE_REQUIREMENT[ante as usize],
        Stake::Gold => PURPLE_STAKE_SCORE_REQUIREMENT[ante as usize],
    };
    let multiplier = match round % 3 {
        1 => 1.0,
        2 => 1.5,
        0 => 2.0,
        _ => unreachable!(),
    };
    (base_score as f32 * multiplier) as u32
}
