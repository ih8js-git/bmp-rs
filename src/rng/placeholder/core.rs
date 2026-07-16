use rand::{Rng, SeedableRng};
use rand_pcg::{Mcg128Xsl64, Pcg64Mcg};

#[derive(Debug, PartialEq, Clone)]
pub struct PrecomputedRngQueue {
    pub storage: Vec<u32>,
}

pub type RNG = Mcg128Xsl64;

pub fn create_generator(seed: u64) -> RNG {
    Pcg64Mcg::seed_from_u64(seed)
}
impl PrecomputedRngQueue {
    /// create queue of length 'size'
    pub fn new(rng: &mut RNG, size: usize) -> PrecomputedRngQueue {
        let mut storage = Vec::with_capacity(size);

        for _ in 0..size {
            storage.push(rng.next_u32());
        }

        PrecomputedRngQueue { storage }
    }

    /// Pull a number directly by its index/offset.
    #[inline(always)] // force compiler to inline
    pub fn get_at_offset(&self, offset: usize) -> u32 {
        self.storage[offset % self.storage.len()] // mod len to stay in bounds
    }
}
