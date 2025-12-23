use std::hash::{Hash, Hasher};
use std::vec;

use rand::distributions::Open01;
use rand::rngs::SmallRng;
use rand::{Rng, RngCore, SeedableRng};
use crate::world::block_kind::Block;
use crate::world::chunk::CHUNK_SIZE;
use super::biomes_def::NUM_BIOMES;
use super::perlin::{PerlinNoiseConfig, MAX_LEVEL_NOISE};

const PROBABILITY_BIOME_CENTER_IN_CHUNK: f32 = 0.05;
const MAX_NUMBER_LAYER: usize = 8;

/// Used to make a list in BiomeConfig, will make the different layers
/// of a biome
#[derive(Clone)]
pub struct BiomeLayer {
    pub start_y_from_top: i32,
    pub block: Block,
}
/// What a biome is.
///
/// Note:
/// - the layers must be ordered from lowest to highest
///     and the None ones should all be after the highest real layer
#[derive(Clone)]
pub struct BiomeConfig {
    pub name: &'static str,
    pub terrain_offset: f32,
    pub terrain_scale: f32,

    pub noise_config: [PerlinNoiseConfig; MAX_LEVEL_NOISE],
    pub layers: [Option<BiomeLayer>; MAX_NUMBER_LAYER],
    pub num_layer: usize,
}

impl BiomeConfig {
    // The constructor enforces that layers respect the convention, allowing for efficient binary search
    pub fn new(
        name: &'static str,
        terrain_offset: f32,
        terrain_scale: f32,
        noise_config: [PerlinNoiseConfig; MAX_LEVEL_NOISE],
        layers: [Option<BiomeLayer>; MAX_NUMBER_LAYER],
        num_layer: usize,
    ) -> Self {
        if num_layer == 0 || (num_layer == 1 && layers[0].is_none()) {
            panic!("Should have at least one layer")
        }

        for i in 0..(num_layer - 1) {
            if let Some(curr_layer) = &layers[i] {
                if let Some(next_layer) = &layers[i + 1] {
                    if next_layer.start_y_from_top <= curr_layer.start_y_from_top {
                        panic!("The layers need to be ordered from lowest to highest!");
                    }
                } else {
                    panic!("There should be no None layers here!");
                }
            } else {
                panic!("There should be no None layers here!");
            }
        }

        Self {
            name,
            terrain_offset,
            terrain_scale,
            noise_config,
            layers,
            num_layer,
        }
    }

    pub fn get_block_at(&self, y: i32) -> Option<Block> {
        if self.layers.is_empty()
            || self.layers[0].is_none()
            || self.layers[0].as_ref().unwrap().start_y_from_top > y
        {
            return None; // No valid position if the first element is already greater than target
        }

        // Thanks to the construction, know for sure there are no None between 0 and num_layer
        let mut low = 0;
        let mut high = self.num_layer;

        // Binary search
        while low < high {
            let mid = (low + high) / 2;
            if self.layers[mid].as_ref().unwrap().start_y_from_top <= y {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        Some(self.layers[low - 1].as_ref().unwrap().block) // The last smaller or equal element will be at `low - 1`
    }
}

/// Manages how biomes are generated, for a given coordinate x,z
/// will return in which biome the position is.
/// The base concept is that we first generate biome centers in the world, each chunk has a certain
/// probability to generate one. Each one is of a randomly chosen biome type.
/// Then, for each block, find the closest biome center and take its type.
pub struct BiomeGenerator {}

impl BiomeGenerator {
    /// Get the chunk position in their grid
    fn get_chunk_coord(world_pos: [i32; 2]) -> [i64; 2] {
        [
            (world_pos[0] / CHUNK_SIZE as i32) as i64,
            (world_pos[1] / CHUNK_SIZE as i32) as i64,
        ]
    }

    /// From a pos in a chunk, compute the world pos
    fn get_world_coord(chunk_coord: [i64; 2], coord_in_chunk: [u64; 2]) -> [i32; 2] {
        let x: i32 = (8 * chunk_coord[0] + coord_in_chunk[0] as i64) as i32;
        let z: i32 = (8 * chunk_coord[1] + coord_in_chunk[1] as i64) as i32;

        [x, z]
    }

    /// For a given chunk, computes if it has a biome center and if it does of what kind and where
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

            (
                Some(Self::get_world_coord(chunk_coord, [x, z])),
                (rng.next_u64() % NUM_BIOMES),
            )
        } else {
            (None, u64::MAX)
        }
    }

    /// Return the ring of a certain level around a chunk
    /// Example: for chunk x, level=1 will return:
    /// o o o
    /// o x o
    /// o o o
    /// the positions of the chunks o
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

    /// For a given world pos, look for the nearest biome center and return its type
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
                let coords = [
                    center_chunk_coord[0] + chunk[0],
                    center_chunk_coord[1] + chunk[1],
                ];
                let (center_opt, biome_type) = BiomeGenerator::get_chunk_biome_center(seed, coords);

                if let Some(center) = center_opt {
                    let diff = [
                        (world_pos[0] - center[0]) as f32,
                        (world_pos[1] - center[1]) as f32,
                    ];
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
    use std::hash::{Hash, Hasher};
    use rand::prelude::SmallRng;
    use rand::{Rng, RngCore, SeedableRng};
    use rand::distributions::Open01;
    use crate::world::block_kind::Block;
    use crate::world::generation::biome::{BiomeConfig, BiomeGenerator, BiomeLayer, MAX_NUMBER_LAYER};
    use crate::world::generation::biomes_def::NUM_BIOMES;
    use crate::world::generation::perlin::PerlinNoiseConfig;

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

        assert!(
            shell.iter().all(|&[x, z]| {
                let max_abs = x.abs().max(z.abs());
                max_abs == level
            }) == true
        );
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

    #[test]
    fn putting_block() {
        let layers: [Option<BiomeLayer>; MAX_NUMBER_LAYER] = [
            Some(BiomeLayer {
                start_y_from_top: 0,
                block: Block::GRASS,
            }),
            Some(BiomeLayer {
                start_y_from_top: 1,
                block: Block::DIRT,
            }),
            Some(BiomeLayer {
                start_y_from_top: 5,
                block: Block::COBBELSTONE,
            }),
            None,
            None,
            None,
            None,
            None,
        ];
        let num_layer = 3;
        let noise_config = [
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
        ];
        let config = BiomeConfig::new("Test", 0.0, 0.0, noise_config, layers, num_layer);

        let cube_height = 20;
        for i in 0..cube_height {
            let block_at_height = config.get_block_at(i);
            if let Some(block) = block_at_height {
                dbg!(block);
            }
        }
    }

    #[test]
    fn test_binary_search() {
        let search_in: [Option<BiomeLayer>; MAX_NUMBER_LAYER] = [
            Some(BiomeLayer {
                start_y_from_top: 0,
                block: Block::DIRT,
            }),
            Some(BiomeLayer {
                start_y_from_top: 10,
                block: Block::GRASS,
            }),
            Some(BiomeLayer {
                start_y_from_top: 20,
                block: Block::OAKLEAVES,
            }),
            None,
            None,
            None,
            None,
            None,
        ];
        let num_layer = 2;
        let noise_config = [
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
            PerlinNoiseConfig {
                scale: 0.0,
                amplitude: 0.0,
            },
        ];
        let config = BiomeConfig::new("Test", 0.0, 0.0, noise_config, search_in, num_layer);

        let y = 9;

        let block = config.get_block_at(y);

        assert_eq!(block, Some(Block::DIRT));
    }
}
