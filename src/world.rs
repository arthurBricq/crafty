use crate::chunk::{Chunk, CHUNK_FLOOR, CHUNK_SIZE};
use crate::cube::Block::COBBELSTONE;
use crate::graphics::cube::CubeAttr;
use crate::vector::Vector3;

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> Self {
        let chunks = Vec::new();
        Self { chunks }
    }
    
    pub fn fill_for_demo(&mut self) {
        let s = CHUNK_SIZE as f32;
        self.chunks.push(Chunk::new_for_demo([0., 0.], 0));
        self.chunks[0].set_cube(Vector3::new(2.0, CHUNK_FLOOR as f32 + 1., 2.), COBBELSTONE);
        self.chunks[0].set_cube(Vector3::new(2.0, CHUNK_FLOOR as f32 + 2., 2.), COBBELSTONE);

        self.chunks.push(Chunk::new_for_demo([s, 0.], 2));
        self.chunks.push(Chunk::new_for_demo([0., -s], 2));
        self.chunks.push(Chunk::new_for_demo([0., s], 2));

        self.chunks.push(Chunk::new_for_demo([-s, 0.], 0));
        self.chunks.push(Chunk::new_for_demo([-2.*s, 0.], 0));
    }

    /// Returns a list of cube attributes to be drawn on the screen.
    /// Each item on this list will result in a cube drawn in the screen.
    ///
    /// 'selected_cube': the currently selected cube, that will be rendered differently.
    pub fn get_cube_attributes(&self, selected_cube: Option<Vector3>) -> Vec<CubeAttr> {
        let mut positions: Vec<CubeAttr> = Vec::new();
        for chunk in &self.chunks {
            // TODO improve this code
            // I know that this is not the best way to do this:
            // 1. It is not optimal ...
            // 2. It breaks the responsibility principle
            for layer in chunk.cubes() {
                for row in layer {
                    for cube in row {
                        if let Some(c) = cube {
                            let is_selected = selected_cube.is_some() && selected_cube.unwrap().equals(&c.position());
                            positions.push(CubeAttr::new(c.model_matrix(), c.block_id(), is_selected));
                        }
                    }
                }
            }
        }
        positions
    }

    /// Returns true if there is a cube at this position
    pub fn is_position_free(&self, pos: &Vector3) -> bool {
        for chunk in &self.chunks {
            if chunk.is_in(pos) {
                if !chunk.is_position_free(pos) {
                    return false;
                }
            }
        }
        true
    }

    /// Returns true if the given position is free falling
    pub fn is_position_free_falling(&self, pos: &Vector3) -> bool {
        for chunk in &self.chunks {
            if chunk.is_in(pos) {
                if !chunk.is_position_free_falling(pos) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::{Chunk, CHUNK_FLOOR, CHUNK_SIZE};
    use crate::vector::Vector3;
    use crate::world::World;

    #[test]
    fn test_chunk_collision_1() {
        let mut world = World::new();
        // Adding one chunk
        let s = CHUNK_SIZE as f32;
        world.chunks.push(Chunk::new_for_demo([-s, 0.], 0));
        world.chunks[0].print_all_cubes();

        // Assert some positions
        assert!(!world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 - 1.5, 4.0)));
        assert!(!world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 - 0.5, 4.0)));
        assert!(!world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 + 0.5, 4.0)));
        assert!(world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 + 1.5, 4.0)));
        assert!(world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 + 1.5, 4.0)));
    }
    
    #[test]
    fn test_chunk_collision_2() {
        let mut world = World::new();
        // Adding one chunk
        world.chunks.push(Chunk::new_for_demo([0., 0.], 0));
        world.chunks[0].print_all_cubes();
        assert!(world.is_position_free(&Vector3::new(4.0, 10.2, 3.0)));
    }
}
