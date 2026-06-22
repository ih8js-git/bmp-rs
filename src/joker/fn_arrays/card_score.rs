use crate::card::Card;
use crate::joker::collection::*;
use crate::joker::{Joker, JokerState};

pub type JokerCardScoreFn = fn(
    state: &JokerState,
    card: &Card,
    chips: &mut f64,
    mult: &mut f64,
) -> Result<(), &'static str>;

fn default_card_score(
    _state: &JokerState,
    _card: &Card,
    _chips: &mut f64,
    _mult: &mut f64,
) -> Result<(), &'static str> {
    Ok(())
}

pub const CARD_SCORE_FNS: [JokerCardScoreFn; 150] = {
    let mut fns: [JokerCardScoreFn; 150] = [default_card_score; 150];
    fns[Joker::GreedyJoker as usize] = greedy_joker::card_score;
    fns
};
