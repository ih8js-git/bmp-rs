use crate::events::NotifyToDelta;
use crate::game::action::GameAction;
use crate::game::delta::GameDelta;
use crate::joker::Joker;

pub const PLANET_FN_IDX: usize = GameAction::UsePlanet { idx: 0 }.index();
pub const PLANET_FNS: [NotifyToDelta; 150] = {
    let mut fns: [NotifyToDelta; 150] = [|_, _| GameDelta::Null; 150];
    fns[Joker::Constellation as usize] = |joker_idx, _| GameDelta::UpdateJokerScaling {
        idx: joker_idx as u8,
        diff: 1,
    };
    fns
};
