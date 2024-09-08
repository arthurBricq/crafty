use crate::block_kind::Block;

use super::{biome::{BiomeConfig, BiomeLayer}, perlin::PerlinNoiseConfig};

/// Number of biomes
pub const NUM_BIOMES: u64 = 4;

/// This file contains all the actual biomes data
pub const BIOMES: [BiomeConfig; NUM_BIOMES as usize] = [
    // Plain biome
    BiomeConfig {
        name: "Plain",
        terrain_offset: 10.,
        terrain_scale: 20.,
        noise_config: [
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
        ],
        layers: [
            Some(BiomeLayer {start_y:  0, block: Block::DIRT}),
            Some(BiomeLayer {start_y: 8, block: Block::GRASS}),
            None, None, None, None, None, None
        ],
        num_layer: 2
    },

    // Plain biome
    BiomeConfig {
        name: "Forest",
        terrain_offset: 10.,
        terrain_scale: 20.,
        noise_config: [
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
        ],
        layers: [
            Some(BiomeLayer {start_y:  0, block: Block::DIRT}),
            Some(BiomeLayer {start_y: 8, block: Block::OAKLOG}),
            None, None, None, None, None, None
        ],
        num_layer: 2
    },

    // Plain biome
    BiomeConfig {
        name: "Mountain",
        terrain_offset: 15.,
        terrain_scale: 40.,
        noise_config: [
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
        ],
        layers: [
            Some(BiomeLayer {start_y:  0, block: Block::COBBELSTONE}),
            None, None, None, None, None, None, None
        ],
        num_layer: 1
    },

    // Plain biome
    BiomeConfig {
        name: "Ocean",
        terrain_offset: 2.,
        terrain_scale: 20.,
        noise_config: [
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
        ],
        layers: [
            Some(BiomeLayer {start_y:  0, block: Block::COBBELSTONE}),
            None, None, None, None, None, None, None
        ],
        num_layer: 1
    },
];

