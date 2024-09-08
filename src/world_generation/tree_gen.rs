use rand::{distributions::Open01, rngs::SmallRng, Rng};

use crate::{block_kind::Block, chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE}, primitives::vector::Vector3};

use super::rng_from_seed::RNGFromSeed;

// Model for the tree: optimally should not be here 
const TREE_STRUCT: [([f32; 3], Block); 33] = [
    // Tronc
    ([0.0, 0.0, 0.0], Block::OAKLOG),
    ([0.0, 1.0, 0.0], Block::OAKLOG),
    ([0.0, 2.0, 0.0], Block::OAKLOG),
    ([0.0, 3.0, 0.0], Block::OAKLOG),

    // Top leaves
    ([0.0, 4.0, 0.0], Block::OAKLEAVES),
    ([1.0, 4.0, 0.0], Block::OAKLEAVES),
    ([-1.0, 4.0, 0.0], Block::OAKLEAVES),
    ([0.0, 4.0, 1.0], Block::OAKLEAVES),
    ([0.0, 4.0, -1.0], Block::OAKLEAVES),

    // Side 1
    ([1.0, 2.0, 0.0], Block::OAKLEAVES),
    ([1.0, 3.0, 0.0], Block::OAKLEAVES),
    ([2.0, 2.0, 0.0], Block::OAKLEAVES),
    ([2.0, 3.0, 0.0], Block::OAKLEAVES),
    ([1.0, 2.0, 1.0], Block::OAKLEAVES),
    ([1.0, 3.0, 1.0], Block::OAKLEAVES),
    ([1.0, 2.0, -1.0], Block::OAKLEAVES),
    ([1.0, 3.0, -1.0], Block::OAKLEAVES),

    // Side 2
    ([-1.0, 2.0, 0.0], Block::OAKLEAVES),
    ([-1.0, 3.0, 0.0], Block::OAKLEAVES),
    ([-2.0, 2.0, 0.0], Block::OAKLEAVES),
    ([-2.0, 3.0, 0.0], Block::OAKLEAVES),
    ([-1.0, 2.0, 1.0], Block::OAKLEAVES),
    ([-1.0, 3.0, 1.0], Block::OAKLEAVES),
    ([-1.0, 2.0, -1.0], Block::OAKLEAVES),
    ([-1.0, 3.0, -1.0], Block::OAKLEAVES),

    // Fill side 3
    ([0.0, 2.0, 1.0], Block::OAKLEAVES),
    ([0.0, 3.0, 1.0], Block::OAKLEAVES),
    ([0.0, 2.0, 2.0], Block::OAKLEAVES),
    ([0.0, 3.0, 2.0], Block::OAKLEAVES),

    // Fill side 4
    ([0.0, 2.0, -1.0], Block::OAKLEAVES),
    ([0.0, 3.0, -1.0], Block::OAKLEAVES),
    ([0.0, 2.0, -2.0], Block::OAKLEAVES),
    ([0.0, 3.0, -2.0], Block::OAKLEAVES),
];

/// Contains various functions required to correctly generate trees
pub struct TreeGenerator {}

impl TreeGenerator {
    /// Compute depending on a given probability if a tree should be there
    /// it is done deterministically
    pub fn should_a_tree_be_here(x: i32, z: i32, seed: u64, biome_prob: f32) -> bool {
        let mut rng: SmallRng = RNGFromSeed::rng_seed_coord(seed, [x as i64, z as i64]);

        rng.sample::<f32, Open01>(Open01) < biome_prob
    }

    /// Will try to plant a tree at a given pos in a chosen chunk
    /// Returns if it succeed
    pub fn try_and_plant_a_tree(tree_pos: Vector3, chunk: &mut Chunk) -> bool {
        // First thing, check the pos is in the middle of the chunk
        let tree_y = tree_pos[1];
        if (tree_y as i32 + 4) >= CHUNK_HEIGHT as i32 {
            return false;
        }

        let tree_x_in_chunk = (tree_pos[0] as i32) % (CHUNK_SIZE as i32);
        if tree_x_in_chunk < 2 || tree_x_in_chunk > 5 {
            return false;
        }

        let tree_z_in_chunk = (tree_pos[2] as i32) % (CHUNK_SIZE as i32);
        if tree_z_in_chunk < 2 || tree_z_in_chunk > 5 {
            return false;
        }

        // Verify the tree space is empty:
        for (block_pos_rel, block) in TREE_STRUCT {
            let block_pos = tree_pos + Vector3::new(block_pos_rel[0], block_pos_rel[1], block_pos_rel[2]);

            if !chunk.is_position_free(&block_pos) {
                return false;
            }
        }

        // We can now place the blocks !
        for (block_pos_rel, block) in TREE_STRUCT {
            let block_pos = tree_pos + Vector3::new(block_pos_rel[0], block_pos_rel[1], block_pos_rel[2]);
            chunk.add_cube(
                block_pos,
                block,
                0,
            );
        }

        return true;
    }
}