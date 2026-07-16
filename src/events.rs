use crate::blinds::Blind;
use crate::card::Card;
use crate::game::action::GameAction;
use crate::game::delta::GameDelta;
use crate::joker::fn_arrays::planet::{PLANET_FN_IDX, PLANET_FNS};
use crate::joker::{JOKER_DEFS, Joker};
use crate::stakes::Stake;
use crate::vouchers::{Voucher, has_voucher};
use crate::{game::state::GameState, score};
use strum::EnumCount;

pub type NotifyToDelta = fn(idx: usize, game_state: &GameState) -> GameDelta;

fn default_fn(_joker_idx: usize, game_state: &GameState) -> GameDelta {
    GameDelta::Null
}
pub const ACTION_TO_FN_ARRAY: [[NotifyToDelta; JOKER_DEFS.len()]; GameAction::COUNT] = {
    let mut fn_arrays = [[default_fn as NotifyToDelta; JOKER_DEFS.len()]; GameAction::COUNT];

    fn_arrays[PLANET_FN_IDX] = PLANET_FNS;

    fn_arrays
};

/// notify listeners of an event and add created deltas to vec
pub fn notify_jokers(deltas: &mut Vec<GameDelta>, action: GameAction, gs: &GameState) {
    for (i, joker) in gs.jokers.iter().enumerate() {
        // check if joker subscribed using bitmask
        let action_idx = action.index();
        let action_mask = 1u64 << action_idx;
        if action_mask & JOKER_EVENT_SUBSCRIPTION_MASKS[joker.id() as usize] != 0 {
            let fn_array = ACTION_TO_FN_ARRAY[action_idx];
            let delta = fn_array[joker.id() as usize](i, gs);
            deltas.push(delta);
        }
    }
}

pub const fn action_to_bitmask(action: GameAction) -> u64 {
    1u64 << action.index()
}

/// Build event subscription masks for all jokers
pub const fn build_event_masks() -> [u64; 150] {
    let mut masks = [0u64; 150];
    let mut i = 0;
    while i < JOKER_DEFS.len() {
        masks[i] = JOKER_DEFS[i].subscribed_to_actions_mask;
        i += 1;
    }
    masks
}

/// Bitmask array indexed by Joker enum value
pub const JOKER_EVENT_SUBSCRIPTION_MASKS: [u64; 150] = build_event_masks();

/// Check if a joker is subscribed to an event
#[inline]
pub fn is_subscribed_to(joker_id: Joker, action: GameAction) -> bool {
    let idx = joker_id as u8 as usize;
    (JOKER_EVENT_SUBSCRIPTION_MASKS[idx] & action_to_bitmask(action)) != 0
}

/// Get all subscribed events for a joker as a bitmask
#[inline]
pub fn joker_event_mask(joker_id: Joker) -> u64 {
    JOKER_EVENT_SUBSCRIPTION_MASKS[joker_id as u8 as usize]
}

pub fn cash_out(gs: &mut GameState) {
    let reward = match gs.next_blind {
        Blind::Small => {
            if gs.stake == Stake::White {
                3
            } else {
                0
            }
        }
        Blind::Big => 4,
        Blind::Boss => 5,
    };
    // TODO add logic for green deck (no interest)
    let mut interest = gs.balance / 5;
    if has_voucher(gs, Voucher::MoneyTree) {
        if interest >= 20 {
            interest = 20;
        }
    } else if has_voucher(gs, Voucher::SeedMoney) {
        if interest >= 10 {
            interest = 10;
        }
    } else if interest >= 5 {
        interest = 5;
    }
    gs.balance += reward + interest + gs.hands_remaining as i32;
}

// TODO: Right now this function is just a wrapper for the get_score function
// but in the future it will do more things. It should be a simple high level
// function that you call that will then call everything it needs to from there
// TODO: either inside or outside this function we need to check if we actually
// have enough hands to play another hand or not, becuase right now nothing is
// stoping us from playing more hands than we actually have
pub fn play_hand(gs: &mut GameState, cards_to_play: [u16; 5]) {
    gs.hands_remaining -= 1;
    let actual_cards: Vec<Card> = cards_to_play
        .iter()
        .map(|&idx| gs.cards[idx as usize])
        .collect();
    gs.current_score = score::core::get_score(gs, &actual_cards);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blinds::Blind::Small;
    use crate::decks::Deck;
    use crate::game::state::create_game_state;
    use crate::joker::Rarity;

    #[test]
    fn test_cash_out() {
        let mut gs = create_game_state(Deck::Red);
        gs.stake = Stake::White;
        gs.next_blind = Small;
        gs.hands_remaining = 3;

        let initial_balance = gs.balance;
        assert_eq!(initial_balance, 4);

        cash_out(&mut gs);

        assert_eq!(gs.balance, 4 + 3 + 0 + 3);
    }

    #[test]
    fn test_joker_with_no_events() {
        // Joker should have no subscriptions
        assert_eq!(joker_event_mask(Joker::Joker), 0);
        assert!(!is_subscribed_to(Joker::Joker, GameAction::PlayBlind));
        assert!(!is_subscribed_to(
            Joker::Joker,
            GameAction::UsePlanet { idx: 0 }
        ));
    }

    #[test]
    fn test_constellation_subscribes_to_planet_used() {
        // Constellation listens to UsePlanet events
        let mask = joker_event_mask(Joker::Constellation);
        assert_ne!(mask, 0);
        assert!(is_subscribed_to(
            Joker::Constellation,
            GameAction::UsePlanet { idx: 0 }
        ));

        // Should not subscribe to other events
        assert!(!is_subscribed_to(
            Joker::Constellation,
            GameAction::PlayBlind
        ));
        assert!(!is_subscribed_to(
            Joker::Constellation,
            GameAction::PlayHand {
                card_indices: [0; 5],
                amount: 0
            }
        ));
    }

    #[test]
    fn test_bitmask_multiple_events() {
        // Test that bitmask correctly combines multiple events
        let event1_mask = action_to_bitmask(GameAction::PlayBlind);
        let event2_mask = action_to_bitmask(GameAction::UseConsumableWithTargets {
            idx: 0,
            amount: 0,
            cards: [0; 3],
        });
        let combined = event1_mask | event2_mask;

        // Verify both bits are set
        assert_eq!(combined & event1_mask, event1_mask);
        assert_eq!(combined & event2_mask, event2_mask);
    }

    #[test]
    fn test_event_masks_array_bounds() {
        // All jokers should have valid mask lookups
        for i in 0..150 {
            let _mask = JOKER_EVENT_SUBSCRIPTION_MASKS[i];
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
            let _result = is_subscribed_to(joker, GameAction::SellJoker { idx: i });
            // Should not panic
        }
    }

    #[test]
    fn test_bitmask_consistency() {
        // Verify that querying with is_subscribed_to is consistent with direct mask operations
        let constellation_mask = joker_event_mask(Joker::Constellation);
        let planet_used_bit = action_to_bitmask(GameAction::UsePlanet { idx: 0 });

        assert_eq!(
            is_subscribed_to(Joker::Constellation, GameAction::UsePlanet { idx: 0 }),
            (constellation_mask & planet_used_bit) != 0
        );
    }

    #[test]
    fn test_event_masks_non_overlapping() {
        // Each GameAction should have a unique bit position
        let discard = action_to_bitmask(GameAction::DiscardHand {
            card_indices: [0; 5],
            amount: 0,
        });
        let hand_played = action_to_bitmask(GameAction::PlayHand {
            card_indices: [0; 5],
            amount: 0,
        });
        let post_hand = action_to_bitmask(GameAction::SellJoker { idx: 0 });
        let planet_used = action_to_bitmask(GameAction::UsePlanet { idx: 0 });

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
            let expected_mask = def.subscribed_to_actions_mask;

            assert_eq!(
                JOKER_EVENT_SUBSCRIPTION_MASKS[i], expected_mask,
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
    }
}
