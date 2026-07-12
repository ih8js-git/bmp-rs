use crate::blinds::Blind;
use crate::card::Card;
use crate::game::delta::GameDelta;
use crate::joker::fn_arrays::planet_used::PLANET_USED_FNS;
use crate::joker::{JOKER_DEFS, Joker};
use crate::stakes::Stake;
use crate::vouchers::{Voucher, has_voucher};
use crate::{game::state::GameState, score};
use strum::EnumCount;
use strum_macros::EnumCount;

pub type EventTriggerToDelta = fn(idx: usize, game_state: &GameState) -> GameDelta;

fn default_fn(_joker_idx: usize, game_state: &GameState) -> GameDelta {
    GameDelta::Null
}
pub const EVENT_TO_FN_ARRAY: [[(fn(idx: usize) -> GameDelta); JOKER_DEFS.len()];
    JokerUpdateEvent::COUNT] = {
    let mut fn_arrays =
        [[default_fn as EventTriggerToDelta; JOKER_DEFS.len()]; JokerUpdateEvent::COUNT];

    fn_arrays[JokerUpdateEvent::PlanetUsed as usize] = PLANET_USED_FNS;

    fn_arrays
};

/// notify listeners of an event and add created deltas to vec
pub fn notify_jokers(deltas: &mut Vec<GameDelta>, event: JokerUpdateEvent, game_state: &GameState) {
    for (i, joker) in game_state.jokers.iter().enumerate() {
        // check if joker subscribed using bitmask
        let event_mask = event_to_bitmask(event);
        if event_mask & JOKER_EVENT_SUBSCRIPTION_MASKS[joker.id() as usize] != 0 {
            let fn_array = EVENT_TO_FN_ARRAY[event as usize];
            let delta = fn_array[joker.id() as usize](i);
            deltas.push(delta);
        }
    }
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
pub const JOKER_EVENT_SUBSCRIPTION_MASKS: [u64; 150] = build_event_masks();

/// Check if a joker is subscribed to an event
#[inline]
pub fn is_subscribed_to(joker_id: Joker, event: JokerUpdateEvent) -> bool {
    let idx = joker_id as u8 as usize;
    (JOKER_EVENT_SUBSCRIPTION_MASKS[idx] & event_to_bitmask(event)) != 0
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
    gs.balance += reward + interest + gs.hands_remaining as u32;
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

        // Verify Constellation has the event
        assert!(
            !JOKER_DEFS[Joker::Constellation as usize]
                .update_events
                .is_empty()
        );
    }
}
