use crate::consumable::simple_consumable_action_to_deltas;
use crate::game::delta::GameDelta;
use crate::game::state::GameState;

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
    UseSimpleConsumable {
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
    pub fn to_deltas(self, gs: &GameState) -> Vec<GameDelta> {
        match self {
            GameAction::MoveJoker { .. } => Vec::new(),
            GameAction::SellJoker { .. } => Vec::new(),
            GameAction::SellConsumable { .. } => Vec::new(),
            GameAction::UseSimpleConsumable { idx } => simple_consumable_action_to_deltas(idx, gs),
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
