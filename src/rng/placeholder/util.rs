use strum::EnumCount;
use crate::decks::Deck;
use crate::game::GameState;
use crate::rng::core::PrecomputedRngQueue;
use crate::rng::placeholder::*;
use crate::rng::queues::RNGQueueType;

/// shuffles deck for the next blind
pub fn shuffle_deck_for_blind(
    gs: &GameState,
) {
    let rand_queue_idx = (gs.next_blind as u8 + gs.ante * 3) as usize;
    // let rand = gs.rng_queues[RNGQueueType::DeckShuffleBlinds][rand_queue_idx];



}