use std::hash::{Hash, Hasher};

use rand::{rngs::SmallRng, SeedableRng};


pub struct RNGFromSeed {}

impl RNGFromSeed {
    pub fn rng_seed_coord(seed: u64, chunk_coord: [i64; 2]) -> SmallRng {
        // Generate a seed for deterministic PRNG
        let mut hasher = std::hash::DefaultHasher::new();
        // Hash the seed
        seed.hash(&mut hasher);
        // Hash the chunk coordinates
        chunk_coord.hash(&mut hasher);
        // Combine the hashes into a final value
        let specific_seed_hash = hasher.finish();  

        SmallRng::seed_from_u64(specific_seed_hash)
    }
}