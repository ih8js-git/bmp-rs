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
    defs[Joker::JollyJoker as usize] = create_def(true, true, true, Rarity::Common, 3);
    defs[Joker::ZanyJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::MadJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::CrazyJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::DrollJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::SlyJoker as usize] = create_def(true, true, true, Rarity::Common, 3);
    defs[Joker::WilyJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::CleverJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::DeviousJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::CraftyJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::HalfJoker as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::JokerStencil as usize] = create_def(true, true, true, Rarity::Uncommon, 8);
    defs[Joker::FourFingers as usize] = create_def(false, true, true, Rarity::Uncommon, 7);
    defs[Joker::Mime as usize] = create_def(true, true, true, Rarity::Uncommon, 5);
    defs[Joker::CreditCard as usize] = create_def(false, true, true, Rarity::Common, 1);
    defs[Joker::CeremonialDagger as usize] = create_def(true, false, true, Rarity::Uncommon, 6);
    defs[Joker::Banner as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::MysticSummit as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::MarbleJoker as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::LoyaltyCard as usize] = create_def(true, true, true, Rarity::Uncommon, 5);
    defs[Joker::_8Ball as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::Misprint as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::Dusk as usize] = create_def(true, true, true, Rarity::Uncommon, 5);
    defs[Joker::RaisedFist as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::ChaosTheClown as usize] = create_def(false, true, true, Rarity::Common, 4);
    defs[Joker::Fibonacci as usize] = create_def(true, true, true, Rarity::Uncommon, 8);
    defs[Joker::SteelJoker as usize] = create_def(true, true, true, Rarity::Uncommon, 7);
    defs[Joker::ScaryFace as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::AbstractJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::DelayedGratification as usize] = create_def(false, true, true, Rarity::Common, 4);
    defs[Joker::Hack as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::Pareidolia as usize] = create_def(false, true, true, Rarity::Uncommon, 5);
    defs[Joker::GrosMichel as usize] = create_def(true, true, false, Rarity::Common, 5);
    defs[Joker::EvenSteven as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::OddTodd as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::Scholar as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::BusinessCard as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::Supernova as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::RideTheBus as usize] = create_def(true, false, true, Rarity::Common, 6);
    defs[Joker::SpaceJoker as usize] = create_def(true, true, true, Rarity::Uncommon, 5);
    defs[Joker::Egg as usize] = create_def(false, true, true, Rarity::Common, 4);
    defs[Joker::Burglar as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::Blackboard as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::Runner as usize] = create_def(true, false, true, Rarity::Common, 5);
    defs[Joker::IceCream as usize] = create_def(true, true, false, Rarity::Common, 5);
    defs[Joker::Dna as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::Splash as usize] = create_def(false, true, true, Rarity::Common, 3);
    defs[Joker::BlueJoker as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::SixthSense as usize] = create_def(false, true, true, Rarity::Uncommon, 6);
    defs[Joker::Constellation as usize] = create_def(true, false, true, Rarity::Uncommon, 6);
    defs[Joker::Hiker as usize] = create_def(true, true, true, Rarity::Uncommon, 5);
    defs[Joker::FacelessJoker as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::GreenJoker as usize] = create_def(true, false, true, Rarity::Common, 4);
    defs[Joker::Superposition as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::ToDoList as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::Cavendish as usize] = create_def(true, true, false, Rarity::Common, 4);
    defs[Joker::CardSharp as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::RedCard as usize] = create_def(true, false, true, Rarity::Common, 5);
    defs[Joker::Madness as usize] = create_def(true, false, true, Rarity::Uncommon, 7);
    defs[Joker::SquareJoker as usize] = create_def(true, false, true, Rarity::Common, 4);
    defs[Joker::Seance as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::RiffRaff as usize] = create_def(true, true, true, Rarity::Common, 6);
    defs[Joker::Vampire as usize] = create_def(true, false, true, Rarity::Uncommon, 7);
    defs[Joker::Shortcut as usize] = create_def(false, true, true, Rarity::Uncommon, 7);
    defs[Joker::Hologram as usize] = create_def(true, false, true, Rarity::Uncommon, 7);
    defs[Joker::Vagabond as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::Baron as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::Cloud9 as usize] = create_def(false, true, true, Rarity::Uncommon, 7);
    defs[Joker::Rocket as usize] = create_def(false, false, true, Rarity::Uncommon, 6);
    defs[Joker::Obelisk as usize] = create_def(true, false, true, Rarity::Rare, 8);
    defs[Joker::MidasMask as usize] = create_def(false, true, true, Rarity::Uncommon, 7);
    defs[Joker::Luchador as usize] = create_def(true, true, false, Rarity::Uncommon, 5);
    defs[Joker::Photograph as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::GiftCard as usize] = create_def(false, true, true, Rarity::Uncommon, 6);
    defs[Joker::TurtleBean as usize] = create_def(false, true, false, Rarity::Uncommon, 6);
    defs[Joker::Erosion as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::ReservedParking as usize] = create_def(true, true, true, Rarity::Common, 6);
    defs[Joker::MailInRebate as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::ToTheMoon as usize] = create_def(false, true, true, Rarity::Uncommon, 5);
    defs[Joker::Hallucination as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::FortuneTeller as usize] = create_def(true, true, true, Rarity::Common, 6);
    defs[Joker::Juggler as usize] = create_def(false, true, true, Rarity::Common, 4);
    defs[Joker::Drunkard as usize] = create_def(false, true, true, Rarity::Common, 4);
    defs[Joker::StoneJoker as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::GoldenJoker as usize] = create_def(false, true, true, Rarity::Common, 6);
    defs[Joker::LuckyCat as usize] = create_def(true, false, true, Rarity::Uncommon, 6);
    defs[Joker::BaseballCard as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::Bull as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::DietCola as usize] = create_def(true, true, false, Rarity::Uncommon, 6);
    defs[Joker::TradingCard as usize] = create_def(false, true, true, Rarity::Uncommon, 6);
    defs[Joker::FlashCard as usize] = create_def(true, false, true, Rarity::Uncommon, 5);
    defs[Joker::Popcorn as usize] = create_def(true, true, false, Rarity::Common, 5);
    defs[Joker::SpareTrousers as usize] = create_def(true, false, true, Rarity::Uncommon, 6);
    defs[Joker::AncientJoker as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::Ramen as usize] = create_def(true, true, false, Rarity::Uncommon, 6);
    defs[Joker::WalkieTalkie as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::Seltzer as usize] = create_def(true, true, false, Rarity::Uncommon, 6);
    defs[Joker::Castle as usize] = create_def(true, false, true, Rarity::Uncommon, 6);
    defs[Joker::SmileyFace as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::Campfire as usize] = create_def(true, true, true, Rarity::Rare, 9);
    defs[Joker::GoldenTicket as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::MrBones as usize] = create_def(false, true, false, Rarity::Uncommon, 5);
    defs[Joker::Acrobat as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::SockAndBuskin as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::Swashbuckler as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::Troubadour as usize] = create_def(false, true, true, Rarity::Uncommon, 6);
    defs[Joker::Certificate as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::SmearedJoker as usize] = create_def(false, true, true, Rarity::Uncommon, 7);
    defs[Joker::Throwback as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::HangingChad as usize] = create_def(true, true, true, Rarity::Common, 4);
    defs[Joker::RoughGem as usize] = create_def(true, true, true, Rarity::Uncommon, 7);
    defs[Joker::Bloodstone as usize] = create_def(true, true, true, Rarity::Uncommon, 7);
    defs[Joker::Arrowhead as usize] = create_def(true, true, true, Rarity::Uncommon, 7);
    defs[Joker::OnyxAgate as usize] = create_def(true, true, true, Rarity::Uncommon, 7);
    defs[Joker::GlassJoker as usize] = create_def(true, false, true, Rarity::Uncommon, 6);
    defs[Joker::Showman as usize] = create_def(false, true, true, Rarity::Uncommon, 5);
    defs[Joker::FlowerPot as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::Blueprint as usize] = create_def(true, true, true, Rarity::Rare, 10);
    defs[Joker::WeeJoker as usize] = create_def(true, false, true, Rarity::Rare, 8);
    defs[Joker::MerryAndy as usize] = create_def(false, true, true, Rarity::Uncommon, 7);
    defs[Joker::OopsAll6s as usize] = create_def(false, true, true, Rarity::Uncommon, 4);
    defs[Joker::TheIdol as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::SeeingDouble as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::Matador as usize] = create_def(true, true, true, Rarity::Uncommon, 7);
    defs[Joker::HitTheRoad as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::TheDuo as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::TheTrio as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::TheFamily as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::TheOrder as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::TheTribe as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::Stuntman as usize] = create_def(true, true, true, Rarity::Rare, 7);
    defs[Joker::InvisibleJoker as usize] = create_def(false, true, false, Rarity::Rare, 8);
    defs[Joker::Brainstorm as usize] = create_def(true, true, true, Rarity::Rare, 10);
    defs[Joker::Satellite as usize] = create_def(false, true, true, Rarity::Uncommon, 6);
    defs[Joker::ShootTheMoon as usize] = create_def(true, true, true, Rarity::Common, 5);
    defs[Joker::DriverSLicense as usize] = create_def(true, true, true, Rarity::Rare, 7);
    defs[Joker::Cartomancer as usize] = create_def(true, true, true, Rarity::Uncommon, 6);
    defs[Joker::Astronomer as usize] = create_def(false, true, true, Rarity::Uncommon, 8);
    defs[Joker::BurntJoker as usize] = create_def(true, true, true, Rarity::Rare, 8);
    defs[Joker::Bootstraps as usize] = create_def(true, true, true, Rarity::Uncommon, 7);
    defs[Joker::Caino as usize] = create_def(true, true, true, Rarity::Legendary, 20);
    defs[Joker::Triboulet as usize] = create_def(true, true, true, Rarity::Legendary, 20);
    defs[Joker::Yorick as usize] = create_def(true, true, true, Rarity::Legendary, 20);
    defs[Joker::Chicot as usize] = create_def(false, true, true, Rarity::Legendary, 20);
    defs[Joker::Perkeo as usize] = create_def(true, true, true, Rarity::Legendary, 20);
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

use strum_macros::FromRepr;

#[derive(Debug, Copy, Clone, PartialEq, FromRepr)]
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
    DriverSLicense,
    Cartomancer,
    Astronomer,
    BurntJoker,
    Bootstraps,
    Caino,
    Triboulet,
    Yorick,
    Chicot,
    Perkeo,
}
