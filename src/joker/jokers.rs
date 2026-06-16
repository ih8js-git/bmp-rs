use modular_bitfield::prelude::*;

#[derive(Specifier, Debug, Copy, Clone, PartialEq)]
#[bits = 2]
pub enum Rarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    Legendary = 3,
}

// TODO: This doesn't really store what the joker actually *does*
// but I'm unsure of how to store that infromation right now,
// but given that we have 22 bits left over, we should have plenty of room
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct JokerDef {
    pub blueprint_compat: bool,
    pub perishable_compat: bool,
    pub eternal_compat: bool,
    pub rarity: Rarity,
    pub base_price: B5,
    #[skip]
    __: B22,
}

/// A clean helper function to initialize JokerDef in a const context.
const fn create_def(
    blueprint: bool,
    perishable: bool,
    eternal: bool,
    rarity: Rarity,
    base_price: u8,
) -> JokerDef {
    let raw: u32 = (blueprint as u32)
        | ((perishable as u32) << 1)
        | ((eternal as u32) << 2)
        | ((rarity as u32 & 0b11) << 3)
        | ((base_price as u32 & 0b11111) << 5);

    // modular-bitfield maps bitfields directly to memory arrays
    JokerDef::from_bytes(raw.to_le_bytes())
}

pub const JOKER_DEFS: [JokerDef; 150] = {
    let mut defs = [JokerDef::new(); 150];

    defs[Joker::Joker as usize] = create_def(true, true, true, Rarity::Common, 2);
    defs[Joker::GreedyJoker as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::LustyJoker as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::WrathfulJoker as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::GluttonousJoker as usize] = create_def(true, true, true, Rarity::Common, 5);

    defs
};

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct JokerState {
    pub id: B8,              // 8 bits for Joker ID Enum value
    pub edition: B3,         // 3 bits for Edition Enum value
    pub is_rental: bool,     // 1 bit
    pub is_perishable: bool, // 1 bit
    pub is_eternal: bool,    // 1 bit
    pub is_pinned: bool,     // 1 bit
    pub sell_value: B5,      // 5 bits
    pub scale: B12,          // 12 bits for scale
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Joker {
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    // ... other jokers
}
