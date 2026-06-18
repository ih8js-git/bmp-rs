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
pub enum TriggerTime {
    PreHand = 0,
    CardScored = 1,
    CardHeld = 2,
    PostHand = 3,
    Other = 4,
}

// TODO: This doesn't really store what the joker actually *does*
// but I'm unsure of how to store that information right now,
// but given that we have 22 bits left over, we should have plenty of room
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct JokerDef {
    pub blueprint_compat: bool,
    pub perishable_compat: bool,
    pub eternal_compat: bool,
    pub rarity: Rarity,
    pub base_price: B5,
    pub trigger_time: TriggerTime,
    #[skip]
    __: B19,
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

/// A clean helper function to initialize JokerDef in a const context.
const fn create_def(
    blueprint: bool,
    perishable: bool,
    eternal: bool,
    rarity: Rarity,
    base_price: u8,
    trigger_time: TriggerTime,
) -> JokerDef {
    let raw: u32 = (blueprint as u32)
        | ((perishable as u32) << 1)
        | ((eternal as u32) << 2)
        | ((rarity as u32 & 0b11) << 3)
        | ((base_price as u32 & 0b11111) << 5)
        | ((trigger_time as u32 & 0b111) << 10);

    // modular-bitfield maps bitfields directly to memory arrays
    JokerDef::from_bytes(raw.to_le_bytes())
}

pub const JOKER_DEFS: [JokerDef; 150] = {
    let mut defs = [JokerDef::new(); 150];
    defs[Joker::Joker as usize] =
        create_def(true, true, true, Rarity::Common, 2, TriggerTime::PostHand);
    defs[Joker::GreedyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::CardScored);
    defs[Joker::LustyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::CardScored);
    defs[Joker::WrathfulJoker as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::GluttonousJoker as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::JollyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 3, TriggerTime::PostHand);
    defs[Joker::ZanyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::MadJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::CrazyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::PostHand);
    defs[Joker::DrollJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::SlyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 3, TriggerTime::Other);
    defs[Joker::WilyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::CleverJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::DeviousJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::PostHand);
    defs[Joker::CraftyJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::HalfJoker as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::JokerStencil as usize] =
        create_def(true, true, true, Rarity::Uncommon, 8, TriggerTime::Other);
    defs[Joker::FourFingers as usize] =
        create_def(false, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Mime as usize] =
        create_def(true, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::CreditCard as usize] =
        create_def(false, true, true, Rarity::Common, 1, TriggerTime::Other);
    defs[Joker::CeremonialDagger as usize] =
        create_def(true, false, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Banner as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::MysticSummit as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::MarbleJoker as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::LoyaltyCard as usize] =
        create_def(true, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::_8Ball as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::Misprint as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Dusk as usize] =
        create_def(true, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::RaisedFist as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::ChaosTheClown as usize] =
        create_def(false, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Fibonacci as usize] =
        create_def(true, true, true, Rarity::Uncommon, 8, TriggerTime::Other);
    defs[Joker::SteelJoker as usize] =
        create_def(true, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::ScaryFace as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::AbstractJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::DelayedGratification as usize] =
        create_def(false, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Hack as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Pareidolia as usize] =
        create_def(false, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::GrosMichel as usize] =
        create_def(true, true, false, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::EvenSteven as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::OddTodd as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Scholar as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::BusinessCard as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Supernova as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::RideTheBus as usize] =
        create_def(true, false, true, Rarity::Common, 6, TriggerTime::Other);
    defs[Joker::SpaceJoker as usize] =
        create_def(true, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::Egg as usize] =
        create_def(false, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Burglar as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Blackboard as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Runner as usize] =
        create_def(true, false, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::IceCream as usize] =
        create_def(true, true, false, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::Dna as usize] = create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Splash as usize] =
        create_def(false, true, true, Rarity::Common, 3, TriggerTime::Other);
    defs[Joker::BlueJoker as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::SixthSense as usize] =
        create_def(false, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Constellation as usize] =
        create_def(true, false, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Hiker as usize] =
        create_def(true, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::FacelessJoker as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::GreenJoker as usize] =
        create_def(true, false, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Superposition as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::ToDoList as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Cavendish as usize] =
        create_def(true, true, false, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::CardSharp as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::RedCard as usize] =
        create_def(true, false, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::Madness as usize] =
        create_def(true, false, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::SquareJoker as usize] =
        create_def(true, false, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Seance as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::RiffRaff as usize] =
        create_def(true, true, true, Rarity::Common, 6, TriggerTime::Other);
    defs[Joker::Vampire as usize] =
        create_def(true, false, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Shortcut as usize] =
        create_def(false, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Hologram as usize] =
        create_def(true, false, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Vagabond as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Baron as usize] = create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Cloud9 as usize] =
        create_def(false, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Rocket as usize] =
        create_def(false, false, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Obelisk as usize] =
        create_def(true, false, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::MidasMask as usize] =
        create_def(false, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Luchador as usize] =
        create_def(true, true, false, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::Photograph as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::GiftCard as usize] =
        create_def(false, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::TurtleBean as usize] =
        create_def(false, true, false, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Erosion as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::ReservedParking as usize] =
        create_def(true, true, true, Rarity::Common, 6, TriggerTime::Other);
    defs[Joker::MailInRebate as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::ToTheMoon as usize] =
        create_def(false, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::Hallucination as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::FortuneTeller as usize] =
        create_def(true, true, true, Rarity::Common, 6, TriggerTime::Other);
    defs[Joker::Juggler as usize] =
        create_def(false, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Drunkard as usize] =
        create_def(false, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::StoneJoker as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::GoldenJoker as usize] =
        create_def(false, true, true, Rarity::Common, 6, TriggerTime::Other);
    defs[Joker::LuckyCat as usize] =
        create_def(true, false, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::BaseballCard as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Bull as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::DietCola as usize] =
        create_def(true, true, false, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::TradingCard as usize] =
        create_def(false, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::FlashCard as usize] =
        create_def(true, false, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::Popcorn as usize] =
        create_def(true, true, false, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::SpareTrousers as usize] =
        create_def(true, false, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::AncientJoker as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Ramen as usize] =
        create_def(true, true, false, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::WalkieTalkie as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Seltzer as usize] =
        create_def(true, true, false, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Castle as usize] =
        create_def(true, false, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::SmileyFace as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Campfire as usize] =
        create_def(true, true, true, Rarity::Rare, 9, TriggerTime::Other);
    defs[Joker::GoldenTicket as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::MrBones as usize] =
        create_def(false, true, false, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::Acrobat as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::SockAndBuskin as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Swashbuckler as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::Troubadour as usize] =
        create_def(false, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Certificate as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::SmearedJoker as usize] =
        create_def(false, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Throwback as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::HangingChad as usize] =
        create_def(true, true, true, Rarity::Common, 4, TriggerTime::Other);
    defs[Joker::RoughGem as usize] =
        create_def(true, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Bloodstone as usize] =
        create_def(true, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Arrowhead as usize] =
        create_def(true, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::OnyxAgate as usize] =
        create_def(true, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::GlassJoker as usize] =
        create_def(true, false, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Showman as usize] =
        create_def(false, true, true, Rarity::Uncommon, 5, TriggerTime::Other);
    defs[Joker::FlowerPot as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Blueprint as usize] =
        create_def(true, true, true, Rarity::Rare, 10, TriggerTime::Other);
    defs[Joker::WeeJoker as usize] =
        create_def(true, false, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::MerryAndy as usize] =
        create_def(false, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::OopsAll6s as usize] =
        create_def(false, true, true, Rarity::Uncommon, 4, TriggerTime::Other);
    defs[Joker::TheIdol as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::SeeingDouble as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Matador as usize] =
        create_def(true, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::HitTheRoad as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::TheDuo as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::TheTrio as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::TheFamily as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::TheOrder as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::TheTribe as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Stuntman as usize] =
        create_def(true, true, true, Rarity::Rare, 7, TriggerTime::Other);
    defs[Joker::InvisibleJoker as usize] =
        create_def(false, true, false, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Brainstorm as usize] =
        create_def(true, true, true, Rarity::Rare, 10, TriggerTime::Other);
    defs[Joker::Satellite as usize] =
        create_def(false, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::ShootTheMoon as usize] =
        create_def(true, true, true, Rarity::Common, 5, TriggerTime::Other);
    defs[Joker::DriversLicense as usize] =
        create_def(true, true, true, Rarity::Rare, 7, TriggerTime::Other);
    defs[Joker::Cartomancer as usize] =
        create_def(true, true, true, Rarity::Uncommon, 6, TriggerTime::Other);
    defs[Joker::Astronomer as usize] =
        create_def(false, true, true, Rarity::Uncommon, 8, TriggerTime::Other);
    defs[Joker::BurntJoker as usize] =
        create_def(true, true, true, Rarity::Rare, 8, TriggerTime::Other);
    defs[Joker::Bootstraps as usize] =
        create_def(true, true, true, Rarity::Uncommon, 7, TriggerTime::Other);
    defs[Joker::Canio as usize] =
        create_def(true, true, true, Rarity::Legendary, 20, TriggerTime::Other);
    defs[Joker::Triboulet as usize] =
        create_def(true, true, true, Rarity::Legendary, 20, TriggerTime::Other);
    defs[Joker::Yorick as usize] =
        create_def(true, true, true, Rarity::Legendary, 20, TriggerTime::Other);
    defs[Joker::Chicot as usize] =
        create_def(false, true, true, Rarity::Legendary, 20, TriggerTime::Other);
    defs[Joker::Perkeo as usize] =
        create_def(true, true, true, Rarity::Legendary, 20, TriggerTime::Other);
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
