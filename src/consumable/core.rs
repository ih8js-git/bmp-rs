use crate::consumable::{Planet, Spectral, Tarot};

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
