use super::perlin::MultiscalePerlinNoise;
use super::perlin::PerlinNoiseConfig;
use crate::block_kind::Block::DIRT;
use crate::block_kind::Block::GRASS;
use crate::chunk::Chunk;
use crate::chunk::CHUNK_FLOOR;
use crate::chunk::CHUNK_SIZE;
use crate::primitives::vector::Vector3;
use crate::world::World;

const BASIC_WORLD_CONF: [PerlinNoiseConfig; 5] = [
    PerlinNoiseConfig {
        scale: 50.,
        amplitude: 1.0,
    },
    PerlinNoiseConfig {
        scale: 25.,
        amplitude: 0.5,
    },
    PerlinNoiseConfig {
        scale: 12.5,
        amplitude: 0.25,
    },
    PerlinNoiseConfig {
        scale: 6.25,
        amplitude: 0.125,
    },
    PerlinNoiseConfig {
        scale: 3.125,
        amplitude: 0.0625,
    },
];

pub struct WorldGenerator {}

/// Class which manages the generation of a new world
impl WorldGenerator {
    /// Creates a simple world with hills
    pub fn create_new_random_world(n_chunks: i32) -> World {
        //let mut noise = PerlinNoise::new(121, 32.);
        let mut noise = MultiscalePerlinNoise::new(42, BASIC_WORLD_CONF.to_vec());

        let s = CHUNK_SIZE as f32;
        let mut chunks = vec![];
        let terrain_scale = 20.;
        let terrain_offset = CHUNK_FLOOR as f32;

        // Yes this is slow, but it will be fine for now
        for i in -n_chunks..n_chunks + 1 {
            for j in -n_chunks..n_chunks + 1 {
                let mut chunk = Chunk::new([i as f32 * s, j as f32 * s]);

                // get the height from the perlin noise for each block
                for x in 0..8 {
                    for z in 0..8 {
                        let height = terrain_offset
                            + terrain_scale
                                * noise.at([i as f32 * s + x as f32, j as f32 * s + z as f32]);

                        let cube_height = height.floor() as i32;

                        for y in 0..cube_height - 1 {
                            chunk.add_cube(
                                Vector3::new(
                                    i as f32 * s + x as f32,
                                    y as f32,
                                    j as f32 * s + z as f32,
                                ),
                                DIRT,
                                0,
                            );
                        }
                        chunk.add_cube(
                            Vector3::new(
                                i as f32 * s + x as f32,
                                cube_height as f32,
                                j as f32 * s + z as f32,
                            ),
                            GRASS,
                            0,
                        );
                    }
                }

                chunks.push(chunk);
            }
        }

        World::new(chunks)
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
