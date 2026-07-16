use crate::card::{Card, Edition};
use crate::consumable::{ConsumableState, Planet};
use crate::game::state::GameState;
use crate::joker::JokerState;
use crate::levels::Hand;
use crate::rng::queues::RNGQueueType;
use crate::vouchers::Voucher;

#[derive(Debug)]
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

#[cfg(test)]
mod delta_tests {
    use super::*;
    use crate::decks::Deck;
    use crate::game::state::create_game_state;

    /// Applies a sequence of deltas step-by-step, taking snapshots of the state
    /// at each step. It then reverts them in reverse order, validating that
    /// every intermediate state matches the snapshot exactly, ultimately
    /// returning the GameState to its absolute original form.
    pub fn assert_deltas_are_perfectly_reversible(initial_state: &GameState, deltas: &[GameDelta])
    where
        GameState: Clone + PartialEq + std::fmt::Debug,
    {
        // 1. Accumulate snapshots as we apply each delta forward
        let mut state_snapshots = Vec::with_capacity(deltas.len() + 1);
        state_snapshots.push(initial_state.clone());

        let mut current_state = initial_state.clone();

        for (i, delta) in deltas.iter().enumerate() {
            delta.apply(&mut current_state);
            state_snapshots.push(current_state.clone());
        }

        // 2. Step backward and revert each delta, performing deep inspections
        for i in (0..deltas.len()).rev() {
            let delta = &deltas[i];

            // Revert the delta applied at step i
            delta.revert(&mut current_state);

            // The state here must match the snapshot taken BEFORE applying step i
            let expected_state = &state_snapshots[i];

            assert_eq!(
                &current_state,
                expected_state,
                "State mismatch during deep inspection! \n\
                 Failed while reverting Delta #{idx} of {total}: {delta:?}\n\
                 Expected State: {expected:#?}\n\
                 Actual Reverted State: {actual:#?}",
                idx = i + 1,
                total = deltas.len(),
                delta = delta,
                expected = expected_state,
                actual = &current_state
            );
        }

        // Final verification: Ensure the state returned to absolute baseline
        assert_eq!(
            &current_state, initial_state,
            "Final reverted state did not match the original initial state."
        );
    }

    #[test]
    fn test_complex_chain_of_deltas() {
        let initial = create_game_state(Deck::Blue);

        let scenario = vec![
            GameDelta::Balance { diff: 15 },
            GameDelta::BaseHands { diff: -1 },
            GameDelta::Balance { diff: -5 },
            GameDelta::RemainingHands { diff: 2 },
        ];

        assert_deltas_are_perfectly_reversible(&initial, &scenario);
    }
}
