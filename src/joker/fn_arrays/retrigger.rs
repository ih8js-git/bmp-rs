use crate::card::Card;
use crate::joker::collection::*;
use crate::joker::{Joker, JokerState};

pub type JokerRetriggerFn = fn(card: &Card, jokers: &[JokerState], state: &JokerState) -> usize;
fn default_retrigger(_card: &Card, _jokers: &[JokerState], _state: &JokerState) -> usize {
    0
}
pub const RETRIGGER_FNS: [JokerRetriggerFn; 150] = {
    let mut fns: [JokerRetriggerFn; 150] = [default_retrigger; 150];
    fns[Joker::SockAndBuskin as usize] = sock_and_buskin::retrigger;
    fns
};
