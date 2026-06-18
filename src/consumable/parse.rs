use crate::consumable::core::{Consumable, ConsumableState};
use crate::consumable::{Planet, Spectral, Tarot};

pub fn parse_consumable_to_text(c: ConsumableState) -> String {
    match c.consumable {
        Consumable::Tarot(t) => parse_tarot_to_text(t),
        Consumable::Planet(p) => parse_planet_to_text(p),
        Consumable::Spectral(s) => parse_spectral_to_text(s),
    }
}

pub fn parse_planet_to_text(p: Planet) -> String {
    p.to_string()
}

pub fn parse_tarot_to_text(t: Tarot) -> String {
    t.to_string()
}

pub fn parse_spectral_to_text(s: Spectral) -> String {
    s.to_string()
}
