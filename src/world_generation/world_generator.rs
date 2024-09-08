use super::biome::BiomeGenerator;
use super::biomes_def::{BIOMES, SINGLE_NOISE_CONFIG, BASE_BIOME_CONFIG};
use super::perlin::MultiscalePerlinNoise;
use super::perlin::PerlinNoiseConfig;
use crate::block_kind::Block;
use crate::block_kind::Block::DIRT;
use crate::block_kind::Block::GRASS;
use crate::block_kind::Block::COBBELSTONE;
use crate::block_kind::Block::OAKLOG;
use crate::block_kind::Block::OAKLEAVES;
use crate::chunk::{self, Chunk};
use crate::chunk::CHUNK_FLOOR;
use crate::chunk::CHUNK_SIZE;
use crate::primitives::vector::Vector3;
use crate::world::World;

/// Class which manages the generation of a new world
pub struct WorldGenerator {}

impl WorldGenerator {
    /// Creates a simple world with hills
    pub fn create_new_random_world(n_chunks: i32) -> World {
        //let mut noise = PerlinNoise::new(121, 32.);
        let seed: u64 = 42;
        let mut noise: MultiscalePerlinNoise = MultiscalePerlinNoise::new(seed, BIOMES[0].noise_config.clone());

        let s = CHUNK_SIZE as f32;
        let mut chunks = vec![];

        // Yes this is slow, but it will be fine for now
        for i in -n_chunks..n_chunks + 1 {
            for j in -n_chunks..n_chunks + 1 {
                let x0 = i as f32 * s;
                let z0 = j as f32 * s;

                chunks.push(WorldGenerator::create_random_chunk(seed, [x0, z0], &mut noise));
            }
        }

        World::new(chunks)
    }

    pub fn create_random_chunk(seed: u64, chunk_coord: [f32; 2], noise: &mut MultiscalePerlinNoise) -> Chunk {
        let x0 = chunk_coord[0];
        let z0 = chunk_coord[1];

        let mut chunk = Chunk::new([x0, z0]);

        // get the height from the perlin noise for each block
        for x in 0..8 {
            for z in 0..8 {
                let biome_t: u64 = BiomeGenerator::find_closest_biome(seed, x + x0 as i32, z + z0 as i32);
                
                let biome_config = &BIOMES[biome_t as usize];

                if SINGLE_NOISE_CONFIG {
                    noise.change_config(BASE_BIOME_CONFIG);
                } else {
                    noise.change_config(biome_config.noise_config.clone());
                }

                let height = biome_config.terrain_offset
                    + biome_config.terrain_scale
                        * noise.at([(x as f32) + x0, (z as f32) + z0]);

                let cube_height = height.floor() as i32;

                for y in 0..cube_height {
                    let block_at_height = biome_config.get_block_at(cube_height - y - 1);

                    if let Some(block) = block_at_height {
                        chunk.add_cube(
                            Vector3::new(
                                (x as f32) + x0,
                                y as f32,
                                (z as f32) + z0,
                            ),
                            block,
                            0,
                        );
                    }
                }
            }
        }

        return chunk;
    }

    /// Creates a basic, flat world. For now this is a simple, flat
    /// grassland, extending `nchunks` in each direction.
    ///
    /// 'n_chunks': number of chunks the flat lands extends in any
    /// direction; i.e., (2nchunks + 1) x (2nchunks + 1) chunks will
    /// be created
    pub fn create_new_flat_world(n_chunks: i32) -> World {
        let s = CHUNK_SIZE as f32;
        let mut chunks = vec![];

        // Yes this is slow, but it will be fine for now
        for i in -n_chunks..n_chunks + 1 {
            for j in -n_chunks..n_chunks + 1 {
                let mut chunk = Chunk::new([i as f32 * s, j as f32 * s]);
                for k in 0..CHUNK_FLOOR {
                    chunk.fill_layer(k, DIRT);
                }
                chunk.fill_layer(CHUNK_FLOOR, GRASS);
                chunks.push(chunk);
            }
        }

        World::new(chunks)
    }
}
