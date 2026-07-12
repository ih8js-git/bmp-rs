use crate::consumable::simple_consumable_action_to_deltas;
use crate::game::delta::GameDelta;
use crate::game::state::GameState;
use strum_macros::EnumCount;

#[derive(Debug, EnumCount)]
pub enum GameAction {
    // Global
    MoveJoker {
        from_idx: u8,
        to_idx: u8,
    },
    SellJoker {
        idx: u8,
    },
    SellConsumable {
        idx: u16,
    },
    UsePlanet {
        idx: u16,
    },
    UseConsumableWithTargets {
        idx: u16,
        amount: u8,
        cards: [u16; 3],
    },

    // Blind Select
    SkipBlind,
    PlayBlind,

    // In blind
    PlayHand {
        card_indices: [u16; 5],
        amount: u8,
    },
    DiscardHand {
        card_indices: [u16; 5],
        amount: u8,
    },
    MoveCard {
        from_idx: u16,
        to_idx: u16,
    },

    // Cashout
    Cashout,

    // Shop
    BuyVoucher {
        idx: u8,
    },
    BuyBoosterPack {
        idx: u8,
    },
    BuyFromShop {
        idx: u8,
    },
    BuyAndUse {
        idx: u8,
    },
    Reroll,
    GoNext,

    // In Booster Pack
    SkipPack,
    SelectFromPack {
        idx: u8,
    },
}

impl GameAction {
    #[inline(always)]
    pub fn index(&self) -> usize {
        match self {
            GameAction::MoveJoker { .. } => 0,
            GameAction::SellJoker { .. } => 1,
            GameAction::SellConsumable { .. } => 2,
            GameAction::UsePlanet { .. } => 3,
            GameAction::UseConsumableWithTargets { .. } => 4,
            GameAction::SkipBlind => 5,
            GameAction::PlayBlind => 6,
            GameAction::PlayHand { .. } => 7,
            GameAction::DiscardHand { .. } => 8,
            GameAction::MoveCard { .. } => 9,
            GameAction::Cashout => 10,
            GameAction::BuyVoucher { .. } => 11,
            GameAction::BuyBoosterPack { .. } => 12,
            GameAction::BuyFromShop { .. } => 13,
            GameAction::BuyAndUse { .. } => 14,
            GameAction::Reroll => 15,
            GameAction::GoNext => 16,
            GameAction::SkipPack => 17,
            GameAction::SelectFromPack { .. } => 18,
        }
    }
}

impl GameAction {
    pub fn to_deltas(self, gs: &GameState) -> Vec<GameDelta> {
        match self {
            GameAction::MoveJoker { .. } => Vec::new(),
            GameAction::SellJoker { .. } => Vec::new(),
            GameAction::SellConsumable { .. } => Vec::new(),
            GameAction::UsePlanet { idx } => simple_consumable_action_to_deltas(idx, gs),
            GameAction::UseConsumableWithTargets { .. } => Vec::new(),
            GameAction::SkipBlind => Vec::new(),
            GameAction::PlayBlind => Vec::new(),
            GameAction::PlayHand { .. } => Vec::new(),
            GameAction::DiscardHand { .. } => Vec::new(),
            GameAction::MoveCard { .. } => Vec::new(),
            GameAction::Cashout => Vec::new(),
            GameAction::BuyVoucher { .. } => Vec::new(),
            GameAction::BuyBoosterPack { .. } => Vec::new(),
            GameAction::BuyFromShop { .. } => Vec::new(),
            GameAction::BuyAndUse { .. } => Vec::new(),
            GameAction::Reroll => Vec::new(),
            GameAction::GoNext => Vec::new(),
            GameAction::SkipPack => Vec::new(),
            GameAction::SelectFromPack { .. } => Vec::new(),
        }
    }
}
