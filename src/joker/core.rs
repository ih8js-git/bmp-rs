use crate::joker::fn_arrays::planet::PLANET_FN_IDX;
use modular_bitfield::prelude::*;
use strum_macros::Display;

#[derive(Specifier, Debug, Copy, Clone, PartialEq)]
#[bits = 2]
pub enum Rarity {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    Legendary = 3,
}

#[derive(Specifier, Debug, Copy, Clone, PartialEq)]
#[bits = 3]
// TODO: update this later with other values like Shop
// right now I need this for scoring logic
pub enum ScoringTriggerTime {
    PreHand = 0,
    CardScored = 1,
    CardHeld = 2,
    PostHand = 3,
    Other = 4,
}

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JokerState {
    pub id: B8,               // 8 bits for Joker ID Enum value
    pub edition: B3,          // 3 bits for Edition Enum value
    pub is_rental: bool,      // 1 bit
    pub is_perishable: bool,  // 1 bit
    pub is_eternal: bool,     // 1 bit
    pub is_pinned: bool,      // 1 bit
    pub added_sell_value: B5, // 5 bits
    pub scale: B12,           // 12 bits for scale
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JokerDef {
    pub blueprint: bool,
    pub perishable: bool,
    pub eternal: bool,
    pub rarity: Rarity,
    pub base_price: u8,
    pub trigger_time: ScoringTriggerTime,
    pub subscribed_to_actions_mask: u64,
}

impl JokerDef {
    const fn new() -> Self {
        JokerDef {
            blueprint: false,
            perishable: false,
            eternal: false,
            rarity: Rarity::Common,
            base_price: 0,
            trigger_time: ScoringTriggerTime::Other,
            subscribed_to_actions_mask: 0u64,
        }
    }
}

pub const JOKER_DEFS: [JokerDef; 150] = {
    let mut defs = [JokerDef::new(); 150];

    defs[Joker::Joker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 2,
        trigger_time: ScoringTriggerTime::PostHand,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GreedyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::CardScored,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::LustyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::CardScored,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::WrathfulJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GluttonousJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::JollyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 3,
        trigger_time: ScoringTriggerTime::PostHand,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::ZanyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::MadJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::CrazyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::PostHand,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::DrollJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SlyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 3,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::WilyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::CleverJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::DeviousJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::PostHand,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::CraftyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::HalfJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::JokerStencil as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::FourFingers as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Mime as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::CreditCard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 1,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::CeremonialDagger as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Banner as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::MysticSummit as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::MarbleJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::LoyaltyCard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::_8Ball as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Misprint as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Dusk as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::RaisedFist as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::ChaosTheClown as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Fibonacci as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SteelJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::ScaryFace as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::AbstractJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::DelayedGratification as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Hack as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Pareidolia as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GrosMichel as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::EvenSteven as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::OddTodd as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Scholar as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::BusinessCard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Supernova as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::RideTheBus as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SpaceJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Egg as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Burglar as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Blackboard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Runner as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::IceCream as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Dna as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Splash as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 3,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::BlueJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SixthSense as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Constellation as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 1u64 << PLANET_FN_IDX,
    };

    defs[Joker::Hiker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::FacelessJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GreenJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Superposition as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::ToDoList as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Cavendish as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::CardSharp as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::RedCard as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Madness as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SquareJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Seance as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::RiffRaff as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Vampire as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Shortcut as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Hologram as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Vagabond as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Baron as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Cloud9 as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Rocket as usize] = JokerDef {
        blueprint: false,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Obelisk as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::MidasMask as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Luchador as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Photograph as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GiftCard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TurtleBean as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Erosion as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::ReservedParking as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::MailInRebate as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::ToTheMoon as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Hallucination as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::FortuneTeller as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Juggler as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Drunkard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::StoneJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GoldenJoker as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::LuckyCat as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::BaseballCard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Bull as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::DietCola as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TradingCard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::FlashCard as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Popcorn as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SpareTrousers as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::AncientJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Ramen as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::WalkieTalkie as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Seltzer as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Castle as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SmileyFace as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Campfire as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 9,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GoldenTicket as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::MrBones as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Acrobat as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SockAndBuskin as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Swashbuckler as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Troubadour as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Certificate as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SmearedJoker as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Throwback as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::HangingChad as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::RoughGem as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Bloodstone as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Arrowhead as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::OnyxAgate as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::GlassJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Showman as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::FlowerPot as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Blueprint as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 10,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::WeeJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::MerryAndy as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::OopsAll6s as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TheIdol as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::SeeingDouble as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Matador as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::HitTheRoad as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TheDuo as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TheTrio as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TheFamily as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TheOrder as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::TheTribe as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Stuntman as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::InvisibleJoker as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: false,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Brainstorm as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 10,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Satellite as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::ShootTheMoon as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::DriversLicense as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Cartomancer as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Astronomer as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::BurntJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Bootstraps as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Canio as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Triboulet as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Yorick as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Chicot as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs[Joker::Perkeo as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        subscribed_to_actions_mask: 0u64,
    };

    defs
};

#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[repr(u8)]
pub enum Joker {
    Joker,
    GreedyJoker,
    LustyJoker,
    WrathfulJoker,
    GluttonousJoker,
    JollyJoker,
    ZanyJoker,
    MadJoker,
    CrazyJoker,
    DrollJoker,
    SlyJoker,
    WilyJoker,
    CleverJoker,
    DeviousJoker,
    CraftyJoker,
    HalfJoker,
    JokerStencil,
    FourFingers,
    Mime,
    CreditCard,
    CeremonialDagger,
    Banner,
    MysticSummit,
    MarbleJoker,
    LoyaltyCard,
    _8Ball,
    Misprint,
    Dusk,
    RaisedFist,
    ChaosTheClown,
    Fibonacci,
    SteelJoker,
    ScaryFace,
    AbstractJoker,
    DelayedGratification,
    Hack,
    Pareidolia,
    GrosMichel,
    EvenSteven,
    OddTodd,
    Scholar,
    BusinessCard,
    Supernova,
    RideTheBus,
    SpaceJoker,
    Egg,
    Burglar,
    Blackboard,
    Runner,
    IceCream,
    Dna,
    Splash,
    BlueJoker,
    SixthSense,
    Constellation,
    Hiker,
    FacelessJoker,
    GreenJoker,
    Superposition,
    ToDoList,
    Cavendish,
    CardSharp,
    RedCard,
    Madness,
    SquareJoker,
    Seance,
    RiffRaff,
    Vampire,
    Shortcut,
    Hologram,
    Vagabond,
    Baron,
    Cloud9,
    Rocket,
    Obelisk,
    MidasMask,
    Luchador,
    Photograph,
    GiftCard,
    TurtleBean,
    Erosion,
    ReservedParking,
    MailInRebate,
    ToTheMoon,
    Hallucination,
    FortuneTeller,
    Juggler,
    Drunkard,
    StoneJoker,
    GoldenJoker,
    LuckyCat,
    BaseballCard,
    Bull,
    DietCola,
    TradingCard,
    FlashCard,
    Popcorn,
    SpareTrousers,
    AncientJoker,
    Ramen,
    WalkieTalkie,
    Seltzer,
    Castle,
    SmileyFace,
    Campfire,
    GoldenTicket,
    MrBones,
    Acrobat,
    SockAndBuskin,
    Swashbuckler,
    Troubadour,
    Certificate,
    SmearedJoker,
    Throwback,
    HangingChad,
    RoughGem,
    Bloodstone,
    Arrowhead,
    OnyxAgate,
    GlassJoker,
    Showman,
    FlowerPot,
    Blueprint,
    WeeJoker,
    MerryAndy,
    OopsAll6s,
    TheIdol,
    SeeingDouble,
    Matador,
    HitTheRoad,
    TheDuo,
    TheTrio,
    TheFamily,
    TheOrder,
    TheTribe,
    Stuntman,
    InvisibleJoker,
    Brainstorm,
    Satellite,
    ShootTheMoon,
    DriversLicense,
    Cartomancer,
    Astronomer,
    BurntJoker,
    Bootstraps,
    Canio,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}
