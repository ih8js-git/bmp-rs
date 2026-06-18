use crate::consumable::{Planet, Tarot};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConsumableType {
    Tarot(Tarot),
    Planet(Planet),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Consumable {
    pub consumable_type: ConsumableType,
    pub added_sell_value: u8,
}
