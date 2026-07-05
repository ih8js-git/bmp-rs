use crate::consumable::{Planet, Spectral, Tarot, use_planet_to_deltas};
use crate::game::delta::GameDelta;
use crate::game::state::GameState;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Consumable {
    Tarot(Tarot),
    Planet(Planet),
    Spectral(Spectral),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ConsumableState {
    pub consumable: Consumable,
    pub added_sell_value: u8,
    pub is_negative: bool,
}

pub fn simple_consumable_action_to_deltas(consumable_idx: u16, gs: &GameState) -> Vec<GameDelta> {
    let consumable_state = &gs.consumables[consumable_idx as usize];

    match consumable_state.consumable {
        Consumable::Planet(p) => use_planet_to_deltas(p, gs),
        Consumable::Tarot(t) => Vec::new(),    // TODO
        Consumable::Spectral(s) => Vec::new(), //TODO
    }
}

pub fn create_tarot_consumable(t: Tarot) -> ConsumableState {
    ConsumableState {
        consumable: Consumable::Tarot(t),
        added_sell_value: 0,
        is_negative: false,
    }
}

pub fn create_planet_consumable(p: Planet) -> ConsumableState {
    ConsumableState {
        consumable: Consumable::Planet(p),
        added_sell_value: 0,
        is_negative: false,
    }
}

pub fn create_spectral_consumable(s: Spectral) -> ConsumableState {
    ConsumableState {
        consumable: Consumable::Spectral(s),
        added_sell_value: 0,
        is_negative: false,
    }
}
