use crate::block_kind::Block;

use super::{biome::{BiomeConfig, BiomeLayer}, perlin::{PerlinNoiseConfig, MAX_LEVEL_NOISE}};

/// Number of biomes
pub const NUM_BIOMES: u64 = 4;

/// This file contains all the actual biomes data
pub const BIOMES: [BiomeConfig; NUM_BIOMES as usize] = [
    // Plain biome
    BiomeConfig {
        name: "Plain",
        terrain_offset: 35.,
        terrain_scale: 10.,
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
            Some(BiomeLayer {start_y_from_top:  0, block: Block::GRASS}),
            Some(BiomeLayer {start_y_from_top: 1, block: Block::DIRT}),
            None, None, None, None, None, None
        ],
        num_layer: 2
    },

    // Plain biome
    BiomeConfig {
        name: "Forest",
        terrain_offset: 35.,
        terrain_scale: 5.,
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
            Some(BiomeLayer {start_y_from_top:  0, block: Block::OAKLOG}),
            Some(BiomeLayer {start_y_from_top: 1, block: Block::DIRT}),
            None, None, None, None, None, None
        ],
        num_layer: 2
    },

    // Plain biome
    BiomeConfig {
        name: "Mountain",
        terrain_offset: 35.,
        terrain_scale: 10.,
        noise_config: [
            PerlinNoiseConfig {
                scale: 75.,
                amplitude: 4.0,
            },
            PerlinNoiseConfig {
                scale: 25.,
                amplitude: 3.,
            },
            PerlinNoiseConfig {
                scale: 10.5,
                amplitude: 1.5,
            },
            PerlinNoiseConfig {
                scale: 6.,
                amplitude: 0.5,
            },
            PerlinNoiseConfig {
                scale: 3.,
                amplitude: 0.3,
            },
        ],
        layers: [
            Some(BiomeLayer {start_y_from_top:  0, block: Block::GRASS}),
            Some(BiomeLayer {start_y_from_top:  1, block: Block::DIRT}),
            Some(BiomeLayer {start_y_from_top:  2, block: Block::STONE}),
            None, None, None, None, None, 
        ],
        num_layer: 3
    },

    // Plain biome
    BiomeConfig {
        name: "Ocean",
        terrain_offset: 30.,
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
            Some(BiomeLayer {start_y_from_top:  0, block: Block::SAND}),
            None, None, None, None, None, None, None
        ],
        num_layer: 1
    },
];

pub const BASE_BIOME_CONFIG: [PerlinNoiseConfig; MAX_LEVEL_NOISE] = [
    PerlinNoiseConfig {
        scale: 75.,
        amplitude: 4.0,
    },
    PerlinNoiseConfig {
        scale: 25.,
        amplitude: 3.,
    },
    PerlinNoiseConfig {
        scale: 10.5,
        amplitude: 1.5,
    },
    PerlinNoiseConfig {
        scale: 6.,
        amplitude: 0.5,
    },
    PerlinNoiseConfig {
        scale: 3.,
        amplitude: 0.3,
    },
];

// If on all biomes use the same config
pub const SINGLE_NOISE_CONFIG: bool = true;