use std::time::Instant;
use crate::actions::Action;
use crate::chunk::{Chunk, CubeIndex, CHUNK_FLOOR, CHUNK_SIZE};
use crate::cube::Block::{GRASS, DIRT, COBBELSTONE, OAKLOG};
use crate::graphics::cube::{CubeAttr, CubeContainer};
use crate::vector::Vector3;
use serde::{Deserialize, Serialize};
use crate::camera::Camera;
use crate::cube::Cube;

#[derive(Serialize, Deserialize)]
pub struct World {
    /// The list of the chunks currently being displayed
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
        self.chunks[0].set_cube(Vector3::new(2.0, CHUNK_FLOOR as f32 + 2., 2.), OAKLOG);
        self.chunks.push(Chunk::new_for_demo([s, 0.], 2));
        self.chunks.push(Chunk::new_for_demo([0., -s], 2));
        self.chunks.push(Chunk::new_for_demo([0., s], 2));
        self.chunks.push(Chunk::new_for_demo([-s, 0.], 0));
        self.chunks.push(Chunk::new_for_demo([-2. * s, 0.], 0));
    }

    /// Creates a new random world. For now this is a simple, flat
    /// grassland, extending `nchunks` in each direction.
    ///
    /// 'n_chunks': number of chunks the flat lands extends in any
    /// direction; i.e., (2nchunks + 1) x (2nchunks + 1) chunks will
    /// be created
    pub fn create_new_random_world(n_chunks: i32) -> Self {
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

        let mut world = Self { chunks };
        world.compute_visible_cubes();
        world
    }

    /// Loads a world from a file.
    pub fn from_file(name: &str) -> Option<Self> {
        match std::fs::read_to_string(name) {
            Ok(data) => Some(serde_json::from_str(&data).unwrap()),
            Err(err) => {
                println!("Could not read: {name} with error: {err}");
                None
            }
        }
    }

    /// Saves the current map to the given file
    pub fn save_to_file(&self, name: &str) {
        // Note: so far I am using `serde_json` but we will be able to change this in the future.
        //       There seems to be many options suited for us: https://serde.rs/#data-formats
        let serialized = serde_json::to_string(self).unwrap();
        match std::fs::write(name, serialized) {
            Ok(_) => println!("Map was saved at {name}"),
            Err(err) => {
                println!("Error while saving {name}: {err}")
            }
        }
    }

    /// Returns a list of cube attributes to be drawn on the screen.
    /// Each item on this list will result in a cube drawn in the screen.
    ///
    /// 'selected_cube': the currently selected cube, that will be rendered differently.
    pub fn get_cubes_to_draw(&self, selected_cube: Option<Vector3>) -> Vec<CubeAttr> {
        // I know that this function looks bad, but... Trust the optimizer
        // I have tried to optimize this shit using a custom class that does not re-allocate everything
        // but it does not improve anything ... So let's keep the simple solution of always calling `push`
        let mut positions: Vec<CubeAttr> = Vec::new();
        for chunk in &self.chunks {
            for layer in chunk.cubes() {
                for row in layer {
                    for cube in row {
                        if let Some(c) = cube {
                            if c.is_visible() {
                                let is_selected = selected_cube.is_some() && selected_cube.unwrap().equals(c.position());
                                positions.push(CubeAttr::new(c.model_matrix(), c.block_id(), is_selected));
                            }
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

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Destroy { at } => self.destroy_cube(at)
        }
    }
    
    fn cube_at_mut(&mut self, pos: Vector3) -> Option<&mut Cube> {
        for chunk in &mut self.chunks {
            if chunk.is_in(&pos) {
                return chunk.cube_at_mut(&pos)
            }
        }
        None
    }

    fn destroy_cube(&mut self, at: Vector3) {
        // Find the chunk where the cube is located
        let mut chunk_index = 0;
        for i in 0..self.chunks.len() {
            if self.chunks[i].is_in(&at) {
                chunk_index = i;
                break;
            }
        }
        
        // Mark all the neighbors cube as visible
        if let Some(cube) = self.chunks[chunk_index].cube_at(&at) {
            for pos in cube.neighbors_positions() {
                // Toggle this position
                if let Some(cube_to_toggle) = self.cube_at_mut(pos) {
                    cube_to_toggle.set_is_visible(true);
                }
            }
        }
        
        self.chunks[chunk_index].destroy_cube(at);
    }

    fn visible_cubes_count(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.visible_cube_count()).sum()
    }

    /// Goes through all the cubes in the world, and sets whether the cube is touching air.
    fn compute_visible_cubes(&mut self) {
        // Note to the reader:
        // This function works in two-step. First, the inside of each chunk is passed through.
        // It is easy because each chunk does not need information about the external world.
        // The second part is much harder: we look at the borders of each chunk. It is harder
        // because in rust, it is not possible to do something like this
        //
        // for chunk in &mut self.chunks {
        //     for pos in chunk.border() {
        //         if self.is_position_free(pos) {...}
        //     }
        // }
        //
        // It is not possible because `self` is borrowed as mutable, so it is impossible to
        // call any function with `self.ANY_FUNCTION(...)`
        // As a consequence of this limitation, I had to split the process in two extra steps:
        // (1) collect the indices of the cubes of the border to be marked as not visible and
        // (2) mark them as not visible.


        // 1. First pass inside each chunk

        for chunk in &mut self.chunks {
            chunk.compute_visible_cubes();
        }

        // 2. Handle the borders of each chunk

        // a. First, determine which cubes needs to be set as not visible.
        let mut indices_to_set_as_not_visible: Vec<Vec<CubeIndex>> = vec![Vec::new(); self.chunks.len()];
        for (i, chunk) in self.chunks.iter().enumerate() {
            let border = chunk.border();
            for index in border {
                if let Some(cube) = chunk.cube_at_index(index) {
                    let position = cube.position();
                    let to_check = cube.neighbors_positions();
                    // If any of the neighbors is free, then the border is visible.
                    let is_visible = to_check.iter().any(|pos| self.is_position_free(&pos));
                    if !is_visible {
                        indices_to_set_as_not_visible[i].push(index);
                    }
                }
            }
        }

        // b. Go through these indices, but now apply the modification that needs to be applied
        for (i, chunk) in self.chunks.iter_mut().enumerate() {
            for index in &indices_to_set_as_not_visible[i] {
                if let Some(cube) = chunk.cube_at_index_mut(*index) {
                    cube.set_is_visible(false);
                }
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::{Chunk, CHUNK_FLOOR, CHUNK_SIZE};
    use crate::cube::Block::GRASS;
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
        assert!(world.is_position_free(&Vector3::new(4.0, 10.2, 3.0)));
    }

    #[test]
    fn test_visible_cube_with_two_chunks_that_touch() {
        let mut world = World::new();
        let mut chunk1 = Chunk::new([0., 0.]);
        chunk1.fill_layer(0, GRASS);
        chunk1.fill_layer(1, GRASS);
        chunk1.fill_layer(2, GRASS);

        let mut chunk2 = Chunk::new([CHUNK_SIZE as f32, 0.]);
        chunk2.fill_layer(0, GRASS);
        chunk2.fill_layer(1, GRASS);
        chunk2.fill_layer(2, GRASS);

        world.chunks.push(chunk1);
        world.chunks.push(chunk2);

        let count1 = world.visible_cubes_count();
        world.compute_visible_cubes();
        let count2 = world.visible_cubes_count();

        assert!(count1 > count2);
        assert_eq!(count1, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE);
        assert_eq!(count2, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE - 1 * (CHUNK_SIZE - 2) * 2 * CHUNK_SIZE);
    }

    #[test]
    fn test_visible_cube_with_two_chunks_that_doesnt_touch() {
        let mut world = World::new();
        let mut chunk1 = Chunk::new([0., 0.]);
        chunk1.fill_layer(0, GRASS);
        chunk1.fill_layer(1, GRASS);
        chunk1.fill_layer(2, GRASS);

        let mut chunk2 = Chunk::new([3.*CHUNK_SIZE as f32, 0.]);
        chunk2.fill_layer(0, GRASS);
        chunk2.fill_layer(1, GRASS);
        chunk2.fill_layer(2, GRASS);

        world.chunks.push(chunk1);
        world.chunks.push(chunk2);

        let count1 = world.visible_cubes_count();
        world.compute_visible_cubes();
        let count2 = world.visible_cubes_count();

        assert!(count1 > count2);
        assert_eq!(count1, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE);

        // In this case, there is no border between the two cubes.
        assert_eq!(count2, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE - 2 * (CHUNK_SIZE - 2) * (CHUNK_SIZE - 2));
    }
}
