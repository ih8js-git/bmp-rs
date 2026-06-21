use crate::rng::core::{PrecomputedRngQueue, RNG};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

type Q = PrecomputedRngQueue;
#[derive(Clone, Copy, EnumIter, EnumCount, Display)]
#[repr(u8)]
pub enum RNGQueueType {
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
        rng_next_queue_indices[next_n_idx] += 1;

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
    use crate::rng::core::create_generator;

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
        let queue = RNGQueueType::Joker.create_queue_with_size(&mut rng, custom_size);
    }
}
