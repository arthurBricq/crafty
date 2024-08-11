use crate::chunk::{Chunk, CHUNK_SIZE};
use crate::cube::CubeAttr;

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> Self {
        let mut chunks = Vec::new();
        let s = (CHUNK_SIZE as f32);
        chunks.push(Chunk::new_for_demo([0., 0.], 0.));
        chunks.push(Chunk::new_for_demo([-s, 0.], 1.));
        chunks.push(Chunk::new_for_demo([s, 0.], 1.));
        chunks.push(Chunk::new_for_demo([0., -s], 1.));
        chunks.push(Chunk::new_for_demo([0., s], 1.));
        Self { chunks }
    }

    /// Returns a list of cube attribute to be drawn on the screen.
    pub fn get_cube_attributes(&self) -> Vec<CubeAttr> {
        let mut positions: Vec<CubeAttr> = Vec::new();
        for chunk in &self.chunks {
            // TODO improve this code
            // I know that this is not the best way to do this:
            // 1. It is not optimal ...
            // 2. It breaks the responsability principle
            for layer in chunk.cubes() {
                for row in layer {
                    for cube in row {
                        if let Some(c) = cube {
                            positions.push(CubeAttr::new(c.model_matrix(), c.block_id()));
                        }
                    }
                }
            }
        }
        positions
    }
    
    pub fn is_position_free(&self, pos: [f32;3]) -> bool {
        true
    }
}