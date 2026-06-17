use crate::consumable::{Planet, Tarot};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConsumableType {
    Tarot(Tarot),
    Planet(Planet),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Consumable {
    pub consumable_type: ConsumableType,
    // pub base_sell_value: u8, // I don't think this is necessary, leaving it commented for now
    pub added_sell_value: u8,
}


