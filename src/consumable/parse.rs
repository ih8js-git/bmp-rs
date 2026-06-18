use crate::consumable::core::{Consumable, ConsumableType};
use crate::consumable::{Planet, Tarot};

pub fn parse_consumable_to_text(c: Consumable) -> String {
    match c.consumable_type {
        ConsumableType::Tarot(t) => parse_tarot_to_text(t),
        ConsumableType::Planet(p) => parse_planet_to_text(p),
    }
}

pub fn parse_planet_to_text(p: Planet) -> String {
    p.to_string()
}

pub fn parse_tarot_to_text(t: Tarot) -> String {
    t.to_string()
}
