use crate::events::JokerUpdateEvent;
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
#[derive(Debug, Copy, Clone)]
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

pub const fn event_to_bitmask(event: JokerUpdateEvent) -> u64 {
    1u64 << (event as u8)
}

/// Build event subscription masks for all jokers
pub const fn build_event_masks() -> [u64; 150] {
    let mut masks = [0u64; 150];
    let mut i = 0;
    while i < JOKER_DEFS.len() {
        let mut mask = 0u64;
        let mut j = 0;
        while j < JOKER_DEFS[i].update_events.len() {
            mask |= 1u64 << (JOKER_DEFS[i].update_events[j] as u8);
            j += 1;
        }
        masks[i] = mask;
        i += 1;
    }
    masks
}

/// Bitmask array indexed by Joker enum value
pub const JOKER_EVENT_MASKS: [u64; 150] = build_event_masks();

/// Check if a joker is subscribed to an event
#[inline]
pub fn is_subscribed_to(joker_id: Joker, event: JokerUpdateEvent) -> bool {
    let idx = joker_id as u8 as usize;
    (JOKER_EVENT_MASKS[idx] & event_to_bitmask(event)) != 0
}

/// Get all subscribed events for a joker as a bitmask
#[inline]
pub fn joker_event_mask(joker_id: Joker) -> u64 {
    JOKER_EVENT_MASKS[joker_id as u8 as usize]
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct JokerDef {
    pub blueprint: bool,
    pub perishable: bool,
    pub eternal: bool,
    pub rarity: Rarity,
    pub base_price: u8,
    pub trigger_time: ScoringTriggerTime,
    pub update_events: &'static [JokerUpdateEvent],
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
            update_events: &[],
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
        update_events: &[],
    };

    defs[Joker::GreedyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::CardScored,
        update_events: &[],
    };

    defs[Joker::LustyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::CardScored,
        update_events: &[],
    };

    defs[Joker::WrathfulJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::GluttonousJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::JollyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 3,
        trigger_time: ScoringTriggerTime::PostHand,
        update_events: &[],
    };

    defs[Joker::ZanyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::MadJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::CrazyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::PostHand,
        update_events: &[],
    };

    defs[Joker::DrollJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SlyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 3,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::WilyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::CleverJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::DeviousJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::PostHand,
        update_events: &[],
    };

    defs[Joker::CraftyJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::HalfJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::JokerStencil as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::FourFingers as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Mime as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::CreditCard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 1,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::CeremonialDagger as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Banner as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::MysticSummit as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::MarbleJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::LoyaltyCard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::_8Ball as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Misprint as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Dusk as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::RaisedFist as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::ChaosTheClown as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Fibonacci as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SteelJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::ScaryFace as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::AbstractJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::DelayedGratification as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Hack as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Pareidolia as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::GrosMichel as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::EvenSteven as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::OddTodd as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Scholar as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::BusinessCard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Supernova as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::RideTheBus as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SpaceJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Egg as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Burglar as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Blackboard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Runner as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::IceCream as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Dna as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Splash as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 3,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::BlueJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SixthSense as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Constellation as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[JokerUpdateEvent::PlanetUsed],
    };

    defs[Joker::Hiker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::FacelessJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::GreenJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Superposition as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::ToDoList as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Cavendish as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::CardSharp as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::RedCard as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Madness as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SquareJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Seance as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::RiffRaff as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Vampire as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Shortcut as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Hologram as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Vagabond as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Baron as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Cloud9 as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Rocket as usize] = JokerDef {
        blueprint: false,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Obelisk as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::MidasMask as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Luchador as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Photograph as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::GiftCard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TurtleBean as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Erosion as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::ReservedParking as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::MailInRebate as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::ToTheMoon as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Hallucination as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::FortuneTeller as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Juggler as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Drunkard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::StoneJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::GoldenJoker as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::LuckyCat as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::BaseballCard as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Bull as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::DietCola as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TradingCard as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::FlashCard as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Popcorn as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SpareTrousers as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::AncientJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Ramen as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::WalkieTalkie as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Seltzer as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Castle as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SmileyFace as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Campfire as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 9,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::GoldenTicket as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::MrBones as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: false,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Acrobat as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SockAndBuskin as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Swashbuckler as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Troubadour as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Certificate as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SmearedJoker as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Throwback as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::HangingChad as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::RoughGem as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Bloodstone as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Arrowhead as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::OnyxAgate as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::GlassJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Showman as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::FlowerPot as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Blueprint as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 10,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::WeeJoker as usize] = JokerDef {
        blueprint: true,
        perishable: false,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::MerryAndy as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::OopsAll6s as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 4,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TheIdol as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::SeeingDouble as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Matador as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::HitTheRoad as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TheDuo as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TheTrio as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TheFamily as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TheOrder as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::TheTribe as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Stuntman as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::InvisibleJoker as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: false,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Brainstorm as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 10,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Satellite as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::ShootTheMoon as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Common,
        base_price: 5,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::DriversLicense as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Cartomancer as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 6,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Astronomer as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::BurntJoker as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Rare,
        base_price: 8,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Bootstraps as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Uncommon,
        base_price: 7,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Canio as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Triboulet as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Yorick as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Chicot as usize] = JokerDef {
        blueprint: false,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
    };

    defs[Joker::Perkeo as usize] = JokerDef {
        blueprint: true,
        perishable: true,
        eternal: true,
        rarity: Rarity::Legendary,
        base_price: 20,
        trigger_time: ScoringTriggerTime::Other,
        update_events: &[],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joker_with_no_events() {
        // Joker should have no subscriptions
        assert_eq!(joker_event_mask(Joker::Joker), 0);
        assert!(!is_subscribed_to(
            Joker::Joker,
            JokerUpdateEvent::BlindSelect
        ));
        assert!(!is_subscribed_to(
            Joker::Joker,
            JokerUpdateEvent::PlanetUsed
        ));
    }

    #[test]
    fn test_constellation_subscribes_to_planet_used() {
        // Constellation listens to PlanetUsed events
        let mask = joker_event_mask(Joker::Constellation);
        assert_ne!(mask, 0);
        assert!(is_subscribed_to(
            Joker::Constellation,
            JokerUpdateEvent::PlanetUsed
        ));

        // Should not subscribe to other events
        assert!(!is_subscribed_to(
            Joker::Constellation,
            JokerUpdateEvent::BlindSelect
        ));
        assert!(!is_subscribed_to(
            Joker::Constellation,
            JokerUpdateEvent::HandPlayed
        ));
    }

    #[test]
    fn test_bitmask_multiple_events() {
        // Test that bitmask correctly combines multiple events
        let event1_mask = event_to_bitmask(JokerUpdateEvent::BlindSelect);
        let event2_mask = event_to_bitmask(JokerUpdateEvent::TarotUsed);
        let combined = event1_mask | event2_mask;

        // Verify both bits are set
        assert_eq!(combined & event1_mask, event1_mask);
        assert_eq!(combined & event2_mask, event2_mask);
    }

    #[test]
    fn test_event_masks_array_bounds() {
        // All jokers should have valid mask lookups
        for i in 0..150 {
            let _mask = JOKER_EVENT_MASKS[i];
            // Should not panic on access
        }
    }

    #[test]
    fn test_is_subscribed_bounds_check() {
        // Should safely handle out-of-bounds joker IDs (though they shouldn't exist)
        // Testing with valid joker range
        for i in 0..10 {
            let joker = match i {
                0 => Joker::Joker,
                1 => Joker::GreedyJoker,
                2 => Joker::LustyJoker,
                3 => Joker::WrathfulJoker,
                4 => Joker::GluttonousJoker,
                5 => Joker::JollyJoker,
                6 => Joker::ZanyJoker,
                7 => Joker::MadJoker,
                8 => Joker::CrazyJoker,
                _ => Joker::DrollJoker,
            };
            let _result = is_subscribed_to(joker, JokerUpdateEvent::Sell);
            // Should not panic
        }
    }

    #[test]
    fn test_bitmask_consistency() {
        // Verify that querying with is_subscribed_to is consistent with direct mask operations
        let constellation_mask = joker_event_mask(Joker::Constellation);
        let planet_used_bit = event_to_bitmask(JokerUpdateEvent::PlanetUsed);

        assert_eq!(
            is_subscribed_to(Joker::Constellation, JokerUpdateEvent::PlanetUsed),
            (constellation_mask & planet_used_bit) != 0
        );
    }

    #[test]
    fn test_event_masks_non_overlapping() {
        // Each JokerUpdateEvent should have a unique bit position
        let discard = event_to_bitmask(JokerUpdateEvent::Discard);
        let hand_played = event_to_bitmask(JokerUpdateEvent::HandPlayed);
        let post_hand = event_to_bitmask(JokerUpdateEvent::Sell);
        let planet_used = event_to_bitmask(JokerUpdateEvent::PlanetUsed);

        // No overlap between any two events
        assert_eq!(discard & hand_played, 0);
        assert_eq!(discard & post_hand, 0);
        assert_eq!(discard & planet_used, 0);
        assert_eq!(hand_played & post_hand, 0);
        assert_eq!(hand_played & planet_used, 0);
        assert_eq!(post_hand & planet_used, 0);
    }

    #[test]
    fn test_build_event_masks_completeness() {
        // Verify that JOKER_EVENT_MASKS was built correctly from JOKER_DEFS
        for (i, def) in JOKER_DEFS.iter().enumerate() {
            let mut expected_mask = 0u64;
            for &event in def.update_events {
                expected_mask |= event_to_bitmask(event);
            }

            assert_eq!(
                JOKER_EVENT_MASKS[i], expected_mask,
                "Mask mismatch for joker at index {}",
                i
            );
        }
    }

    #[test]
    fn test_common_jokers_have_correct_defs() {
        // Spot check a few jokers
        assert_eq!(JOKER_DEFS[Joker::Joker as usize].base_price, 2);
        assert_eq!(JOKER_DEFS[Joker::GreedyJoker as usize].base_price, 5);
        assert_eq!(
            JOKER_DEFS[Joker::Constellation as usize].rarity,
            Rarity::Uncommon
        );

        // Verify Constellation has the event
        assert!(
            !JOKER_DEFS[Joker::Constellation as usize]
                .update_events
                .is_empty()
        );
    }
}
