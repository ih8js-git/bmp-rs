use crate::game::GameState;
use crate::rng::queues::RNGQueueType;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

/// shuffles deck for the next blind
pub fn shuffle_deck_before_blind(gs: &mut GameState) {
    // sort the cards by id
    gs.deck.sort_by(|a, b| {
        let length_comparison = a.id.cmp(&b.id);

        if length_comparison == std::cmp::Ordering::Equal {
            a.id.cmp(&b.id)
        } else {
            length_comparison
        }
    });

    // shuffle deck
    let rand_queue_idx = (gs.next_blind as u8 + gs.ante * 3) as usize;
    let rand = gs.rng_queues[RNGQueueType::DeckShuffleBlinds as usize].storage[rand_queue_idx];
    let mut rng = StdRng::seed_from_u64(rand as u64);

    gs.deck.shuffle(&mut rng);
}
