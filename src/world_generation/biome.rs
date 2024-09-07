use std::hash::{Hash, Hasher};
use std::vec;

use rand::distributions::Open01;
use rand::rngs::SmallRng;
use rand::{Rng, RngCore, SeedableRng};

use crate::chunk::CHUNK_SIZE;

const NUM_BIOMES: u64 = 4;
const PROBABILITY_BIOME_CENTER_IN_CHUNK: f32 = 0.05;

/// Manages how biomes are generated, for a given coordinate x,z
/// will return in which biome the position is
pub struct BiomeGenerator {}

impl BiomeGenerator {
    fn get_chunk_coord(world_pos: [i32; 2]) -> [i64; 2] {
        [(world_pos[0] / CHUNK_SIZE as i32) as i64,
            (world_pos[1] / CHUNK_SIZE as i32) as i64]
    }

    fn get_world_coord(chunk_coord: [i64; 2], coord_in_chunk: [u64; 2]) -> [i32; 2] {
        let x: i32 = (8 * chunk_coord[0] + coord_in_chunk[0] as i64) as i32;
        let z: i32 = (8 * chunk_coord[1] + coord_in_chunk[1] as i64) as i32;
        
        [x, z]
    }

    fn get_chunk_biome_center(seed: u64, chunk_coord: [i64; 2]) -> (Option<[i32; 2]>, u64) {
        // Generate a seed for deterministic PRNG
        let mut hasher = std::hash::DefaultHasher::new();
        // Hash the seed
        seed.hash(&mut hasher);
        // Hash the chunk coordinates
        chunk_coord.hash(&mut hasher);
        // Combine the hashes into a final value
        let specific_seed_hash = hasher.finish();  

        let mut rng: SmallRng = SmallRng::seed_from_u64(specific_seed_hash);

        if rng.sample::<f32, Open01>(Open01) < PROBABILITY_BIOME_CENTER_IN_CHUNK {
            let x = rng.next_u64() % 8;
            let z = rng.next_u64() % 8;

            (Some(Self::get_world_coord(chunk_coord, [x, z])),
                (rng.next_u64() % NUM_BIOMES))
        } else {
            (None, u64::MAX)
        }
    }

    fn get_chunk_shell(level: i64) -> Vec<[i64; 2]> {
        let num_side: i64 = 2 * level + 1;
        let half_side: i64 = num_side / 2;

        let mut chunks_in_shell: Vec<[i64; 2]> = vec![];

        for i in 0..num_side {
            let x: i64 = i - half_side;

            if x.abs() == half_side {
                for j in 0..num_side {
                    let z: i64 = j - half_side;
                    chunks_in_shell.push([x, z]);
                }
            } else {
                chunks_in_shell.push([x, half_side]);
                chunks_in_shell.push([x, -half_side]);
            }
        }

        chunks_in_shell
    }

    pub fn find_closest_biome(seed: u64, x: i32, z: i32) -> u64 {
        let world_pos: [i32; 2] = [x, z];    
        let mut current_level: i64 = 0;

        let center_chunk_coord = Self::get_chunk_coord(world_pos);

        let mut closest_biome_type: u64 = u64::MAX;
        let mut shortest_dist: f32 = f32::MAX;

        loop {
            let shell = Self::get_chunk_shell(current_level);

            let mut found_better: bool = false;

            for chunk in shell {
                let coords = [center_chunk_coord[0] + chunk[0],
                                        center_chunk_coord[1] + chunk[1]];
                let (center_opt, biome_type) = BiomeGenerator::get_chunk_biome_center(seed, coords);

                if let Some(center) = center_opt {
                    let diff = [(world_pos[0] - center[0]) as f32,
                                            (world_pos[1] - center[1]) as f32];
                    let new_dist = diff[0].abs() + diff[1].abs();

                    if new_dist < shortest_dist {
                        found_better = true;
                        shortest_dist = new_dist;
                        closest_biome_type = biome_type;
                    }
                }
            }

            if (closest_biome_type != u64::MAX) && (!found_better) {
                return closest_biome_type;
            }

            current_level += 1;
        }
    }

}


#[cfg(test)]
mod tests {
use crate::world_generation::biome::*;

    #[test]
    fn test_randint() {
        let seed: u64 = 42;
        let chunk_coord: [i64; 2] = [0, 1];

        let mut hasher = std::hash::DefaultHasher::new();
        // Hash the seed
        seed.hash(&mut hasher);
        // Hash the chunk coordinates
        chunk_coord.hash(&mut hasher);
        // Combine the hashes into a final value
        let specific_seed_hash = hasher.finish();  

        let mut rng: SmallRng = SmallRng::seed_from_u64(specific_seed_hash);

        let x = rng.next_u64() % 8;

        assert!(x < 7);
    }

    #[test]
    fn check_determinism_biome_center() {
        
        let seed: u64 = 42;
        let coords: [i64; 2] = [0, 2];

        let (center1, biome_type1) = BiomeGenerator::get_chunk_biome_center(seed, coords);
        let (center2, biome_type2) = BiomeGenerator::get_chunk_biome_center(seed, coords);

        assert_eq!(center1, center2);
        assert_eq!(biome_type1, biome_type2)
    }

    #[test]
    fn test_get_chunk_shell_size() {
        let level: i64 = 0;
        let shell = BiomeGenerator::get_chunk_shell(level);
        assert!(shell.len() == 1);
        
        let level: i64 = 1;
        let shell = BiomeGenerator::get_chunk_shell(level);
        assert!(shell.len() == 8);

        let level: i64 = 2;
        let shell = BiomeGenerator::get_chunk_shell(level);
        assert!(shell.len() == 16);
    }
    #[test]
    fn test_max_abs_equal() {
        let level: i64 = 3;
        let shell = BiomeGenerator::get_chunk_shell(level);

        assert!(shell.iter().all(|&[x, z]| {
            let max_abs = x.abs().max(z.abs());
            max_abs == level
        }) == true);
    }

    #[test]
    fn test_find_closest_biome() {
        let seed: u64 = 42;
        let x: i32 = -1;
        let z: i32 = 10;

        let biome_t1 = BiomeGenerator::find_closest_biome(seed, x, z);
        let biome_t2 = BiomeGenerator::find_closest_biome(seed, x, z);

        assert_eq!(biome_t1, biome_t2);
    }



    #[test]
    fn test_randomness() {
        for i in 0..5 {
            for j in 0..5 {
                let seed: u64 = 42;
                let chunk_coord: [i64; 2] = [i, j];
                // Generate a seed for deterministic PRNG
                let mut hasher = std::hash::DefaultHasher::new();
                // Hash the seed
                seed.hash(&mut hasher);
                // Hash the chunk coordinates
                chunk_coord.hash(&mut hasher);
                // Combine the hashes into a final value
                let specific_seed_hash = hasher.finish();  

                let mut rng: SmallRng = SmallRng::seed_from_u64(specific_seed_hash);

                let n = rng.sample::<f32, Open01>(Open01);
                let x = rng.next_u64() % 8;
                let z = rng.next_u64() % 8;
                let t = rng.next_u64() % NUM_BIOMES;
                
                println!("n= {}  x= {}  z= {}  t= {}", n, x, z, t);
            }
        }
    }
}