use crate::block_kind::Block;
use crate::chunk::Chunk;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct SerializedWorld {
    pub chunk_corners: Vec<[f32; 2]>,
    pub cubes_by_kind: HashMap<Block, Vec<[i32; 4]>>,
}

/// Returns a container to be used to serialize chunks or world.
pub fn get_serialize_container() -> HashMap<Block, Vec<[i32; 4]>> {
    let mut all_cubes = HashMap::new();
    for block_kind in Block::iter() {
        all_cubes.insert(block_kind, Vec::<[i32; 4]>::new());
    }
    all_cubes
}

pub fn serialize_one_chunk(all_cubes: &mut HashMap<Block, Vec<[i32; 4]>>, chunk: &Chunk) {
    for cube in chunk.cubes_iter() {
        if let Some(cube) = cube {
            // we can trust that the block has a container.
            let container = all_cubes.get_mut(cube.block()).unwrap();
            container.push([
                cube.position().x() as i32,
                cube.position().y() as i32,
                cube.position().z() as i32,
                cube.n_neighbors() as i32,
            ]);
        }
    }
}
