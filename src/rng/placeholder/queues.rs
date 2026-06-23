use crate::rng::placeholder::core::{PrecomputedRngQueue, RNG};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

type Q = PrecomputedRngQueue;
#[derive(Clone, Copy, EnumIter, EnumCount, Display)]
#[repr(u8)]
pub enum RNGQueueType {
    // Deck
    DeckShuffleBlinds,
    DeckShufflePacks,

    // Shop
    Joker,
    Tarot,
    Planet,
    BoosterPacks,
    Voucher,
    Spectral,
    Judgement,

    // Cards
    Lucky,
    Glass,

    // Consumables
    Aura,
    WheelOfFortune,

    // Jokers
    Bloodstone,
    ToDoList,
    Rebate,
    Idol,
    Certificate,
    ReservedParking,
    BusinessCard,
    _8Ball,
    Ancient,

    // Blind select
    SkipTag,
    BossBlind,
}

impl RNGQueueType {
    /// how large the queue is, should cover the max values of 99.99...% of games played
    fn size(&self) -> usize {
        match self {
            RNGQueueType::Joker => 100_000,
            RNGQueueType::Tarot => 100_000,
            RNGQueueType::Planet => 100_000,
            RNGQueueType::BoosterPacks => 1000,
            RNGQueueType::Voucher => 64,
            RNGQueueType::Spectral => 10_000,
            RNGQueueType::Judgement => 1000,
            RNGQueueType::Lucky => 1_000_000,
            RNGQueueType::Glass => 1_000_000,
            RNGQueueType::Aura => 1_000,
            RNGQueueType::WheelOfFortune => 1_000,
            RNGQueueType::Bloodstone => 1_000_000,
            RNGQueueType::ToDoList => 1_000,
            RNGQueueType::Rebate => 100,
            RNGQueueType::SkipTag => 100,
            RNGQueueType::BossBlind => 100,
            RNGQueueType::Idol => 100,
            _ => 100_000,
        }
    }

    /// PrecomputedRngQueue with the default size for this queue type
    pub fn create_queue(&self, rng: &mut RNG) -> PrecomputedRngQueue {
        PrecomputedRngQueue::new(rng, self.size())
    }

    /// Creates a PrecomputedRngQueue with a custom size override
    pub fn create_queue_with_size(&self, rng: &mut RNG, custom_size: usize) -> PrecomputedRngQueue {
        PrecomputedRngQueue::new(rng, custom_size)
    }

    pub fn get_next(
        &self,
        rng_queues: &[PrecomputedRngQueue; RNGQueueType::COUNT],
        rng_next_queue_indices: &mut [u32; Self::COUNT],
    ) -> u32 {
        let queue_idx = *self as usize;
        let queue = &rng_queues[queue_idx];

        let next_n_idx = rng_next_queue_indices[queue_idx] as usize;
        let n = queue.storage[next_n_idx];
        rng_next_queue_indices[queue_idx] += 1;

        n
    }
}

pub fn create_all_rng_queues(rng: &mut RNG) -> [PrecomputedRngQueue; RNGQueueType::COUNT] {
    let mut variants = RNGQueueType::iter();

    std::array::from_fn(|_| {
        let variant = variants.next().expect("Iterator should match COUNT");
        variant.create_queue(rng)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::placeholder::core::create_generator;

    #[test]
    fn test_create_all_queues_length() {
        let mut rng = create_generator(123);
        let queues = create_all_rng_queues(&mut rng);

        assert_eq!(queues.len(), RNGQueueType::COUNT);
    }

    #[test]
    fn test_custom_size_override() {
        let mut rng = create_generator(123);
        let custom_size = 500;
        let _queue = RNGQueueType::Joker.create_queue_with_size(&mut rng, custom_size);
    }

    #[test]
    fn test_get_next_advances_and_returns_correct_value() {
        let mut rng = create_generator(42);
        let queues = create_all_rng_queues(&mut rng);
        let mut indices = [0u32; RNGQueueType::COUNT];

        // Capture what the first two values should be manually from the queue
        let joker_idx = RNGQueueType::Joker as usize;
        let expected_first = queues[joker_idx].storage[0];
        let expected_second = queues[joker_idx].storage[1];

        // Act & Assert first pull
        let first = RNGQueueType::Joker.get_next(&queues, &mut indices);
        assert_eq!(first, expected_first);
        assert_eq!(indices[joker_idx], 1, "Index should have incremented to 1");

        // Act & Assert second pull
        let second = RNGQueueType::Joker.get_next(&queues, &mut indices);
        assert_eq!(second, expected_second);
        assert_eq!(indices[joker_idx], 2, "Index should have incremented to 2");
    }

    #[test]
    fn test_get_next_independent_queues() {
        let mut rng = create_generator(999);
        let queues = create_all_rng_queues(&mut rng);
        let mut indices = [0u32; RNGQueueType::COUNT];

        let joker_idx = RNGQueueType::Joker as usize;
        let tarot_idx = RNGQueueType::Tarot as usize;

        // Pull from Joker
        RNGQueueType::Joker.get_next(&queues, &mut indices);
        assert_eq!(indices[joker_idx], 1);
        assert_eq!(indices[tarot_idx], 0, "Tarot index should remain untouched");

        // Pull from Tarot
        RNGQueueType::Tarot.get_next(&queues, &mut indices);
        assert_eq!(indices[joker_idx], 1, "Joker index should remain unchanged");
        assert_eq!(indices[tarot_idx], 1);
    }
}
