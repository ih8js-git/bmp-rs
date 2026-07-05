use crate::game::delta::GameDelta;
use crate::joker::Joker;

pub type PlanetUsedJokerUpdate = fn(idx: u8) -> GameDelta;

fn default_fn(_joker_idx: u8) -> GameDelta {
    GameDelta::Null
}

pub const PLANET_USED_FNS: [PlanetUsedJokerUpdate; 150] = {
    let mut fns: [PlanetUsedJokerUpdate; 150] = [default_fn; 150];
    fns[Joker::Constellation as usize] = |joker_idx| GameDelta::UpdateJokerScaling {
        idx: joker_idx,
        diff: 1,
    };
    fns
};
