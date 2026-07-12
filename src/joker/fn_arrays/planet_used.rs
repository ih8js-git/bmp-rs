use crate::events::EventTriggerToDelta;
use crate::game::delta::GameDelta;
use crate::joker::Joker;

pub const PLANET_USED_FNS: [EventTriggerToDelta; 150] = {
    let mut fns: [EventTriggerToDelta; 150] = [|_, _| GameDelta::Null; 150];
    fns[Joker::Constellation as usize] = |joker_idx, _| GameDelta::UpdateJokerScaling {
        idx: joker_idx as u8,
        diff: 1,
    };
    fns
};
