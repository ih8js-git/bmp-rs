use crate::card::{Card, Edition};
use crate::consumable::{ConsumableState, Planet};
use crate::game::state::GameState;
use crate::joker::JokerState;
use crate::levels::Hand;
use crate::rng::queues::RNGQueueType;
use crate::vouchers::Voucher;

pub enum GameDelta {
    Null, // nothing happens, used for placeholders
    EctoUsed,
    SkipBlind,
    Balance {
        diff: i32,
    },
    HandSize {
        diff: i8,
    },
    JokerSlots {
        diff: i8,
    },
    AddConsumable {
        consumable_state: ConsumableState,
    },
    RemoveConsumable {
        consumable_state: ConsumableState,
        idx: u16,
    },
    Planet {
        planet: Planet,
        diff: i8,
    },

    // Cards
    AddCardToDeck {
        card: Card,
    },
    RemoveCardFromDeck {
        card: Card,
        idx: u16,
    },
    UpdateCardFull {
        idx: u16,
        old_meta: u16,
        new_meta: u16,
        old_flags: u8,
        new_flags: u8,
        chips_diff: i16,
        id_diff: i16,
    },

    RemoveJoker {
        joker_state: JokerState,
        idx: u8,
    },

    AddJoker {
        joker_state: JokerState,
    },

    UpdateJokerScaling {
        idx: u8,
        diff: i16,
    },

    UpdateJokerEdition {
        idx: u8,
        old: Edition,
        new: Edition,
    },

    MoveJoker {
        from_idx: u8,
        to_idx: u8,
    },

    BuyVoucher {
        voucher: Voucher,
    },
    HandTypesPlayed {
        hand: Hand,
    },

    RNGQueue {
        queue: RNGQueueType,
        idx_diff: i32,
    },

    BaseRerollCost {
        diff: i8,
    },

    Ante {
        diff: i8,
    },
    NextBlind {
        diff: i8,
    },
    BaseHands {
        diff: i8,
    },
    BaseDiscards {
        diff: i8,
    },
    RemainingHands {
        diff: i8,
    },
    RemainingDiscards {
        diff: i8,
    },
    CurrentScore {
        diff: f64,
    },
}

impl GameDelta {
    pub fn apply(&self, gs: &mut GameState) {
        match self {
            GameDelta::Null => (),
            GameDelta::EctoUsed => {}
            GameDelta::SkipBlind => {}
            GameDelta::Balance { .. } => {}
            GameDelta::HandSize { .. } => {}
            GameDelta::JokerSlots { .. } => {}
            GameDelta::AddConsumable { .. } => {}
            GameDelta::RemoveConsumable { .. } => {}
            GameDelta::AddCardToDeck { .. } => {}
            GameDelta::RemoveCardFromDeck { .. } => {}
            GameDelta::UpdateCardFull { .. } => {}
            GameDelta::RemoveJoker { .. } => {}
            GameDelta::AddJoker { .. } => {}
            GameDelta::UpdateJokerScaling { .. } => {}
            GameDelta::UpdateJokerEdition { .. } => {}
            GameDelta::MoveJoker { .. } => {}
            GameDelta::BuyVoucher { .. } => {}
            GameDelta::HandTypesPlayed { .. } => {}
            GameDelta::RNGQueue { .. } => {}
            GameDelta::BaseRerollCost { .. } => {}
            GameDelta::Ante { .. } => {}
            GameDelta::NextBlind { .. } => {}
            GameDelta::BaseHands { .. } => {}
            GameDelta::BaseDiscards { .. } => {}
            GameDelta::RemainingHands { .. } => {}
            GameDelta::RemainingDiscards { .. } => {}
            GameDelta::CurrentScore { .. } => {}
            GameDelta::Planet { .. } => {}
        }
    }

    pub fn revert(&self, gs: &mut GameState) {
        match self {
            GameDelta::Null => {}
            GameDelta::EctoUsed => {}
            GameDelta::SkipBlind => {}
            GameDelta::Balance { .. } => {}
            GameDelta::HandSize { .. } => {}
            GameDelta::JokerSlots { .. } => {}
            GameDelta::AddConsumable { .. } => {}
            GameDelta::RemoveConsumable { .. } => {}
            GameDelta::AddCardToDeck { .. } => {}
            GameDelta::RemoveCardFromDeck { .. } => {}
            GameDelta::UpdateCardFull { .. } => {}
            GameDelta::RemoveJoker { .. } => {}
            GameDelta::AddJoker { .. } => {}
            GameDelta::UpdateJokerScaling { .. } => {}
            GameDelta::UpdateJokerEdition { .. } => {}
            GameDelta::MoveJoker { .. } => {}
            GameDelta::BuyVoucher { .. } => {}
            GameDelta::HandTypesPlayed { .. } => {}
            GameDelta::RNGQueue { .. } => {}
            GameDelta::BaseRerollCost { .. } => {}
            GameDelta::Ante { .. } => {}
            GameDelta::NextBlind { .. } => {}
            GameDelta::BaseHands { .. } => {}
            GameDelta::BaseDiscards { .. } => {}
            GameDelta::RemainingHands { .. } => {}
            GameDelta::RemainingDiscards { .. } => {}
            GameDelta::CurrentScore { .. } => {}
            GameDelta::Planet { .. } => {}
        }
    }
}
