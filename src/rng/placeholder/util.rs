use crate::game::state::GameState;
use crate::rng::queues::RNGQueueType;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

/// shuffles deck for the next blind
pub fn shuffle_deck_before_blind(gs: &mut GameState) {
    // sort draw_pile indices by card id
    gs.draw_pile.sort_by(|&a, &b| {
        let ca = &gs.cards[a as usize];
        let cb = &gs.cards[b as usize];
        ca.id.cmp(&cb.id)
    });

    // shuffle draw_pile
    let rand_queue_idx = (gs.next_blind as u8 + gs.ante * 3) as usize;
    let rand = gs.rng_queues[RNGQueueType::DeckShuffleBlinds as usize].storage[rand_queue_idx];
    let mut rng = StdRng::seed_from_u64(rand as u64);

    gs.draw_pile.shuffle(&mut rng);
}
