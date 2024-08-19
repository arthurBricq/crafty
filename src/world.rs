use crate::actions::Action;
use crate::chunk::{Chunk, CHUNK_FLOOR, CHUNK_SIZE};
use crate::cube::Block::{COBBELSTONE, OAKLOG};
use crate::graphics::cube::CubeAttr;
use crate::vector::Vector3;
use serde::{Deserialize, Serialize};

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
        self.chunks.push(Chunk::new_for_demo([-2.*s, 0.], 0));
    }
    
    /// Creates a new random world
    pub fn create_new_random_world() -> Self {
        // TODO [Johan]
        //      I think that using the factory pattern here would be good, 
        //      eg. create a new `WorldFactory` struct in a another file.
        Self {chunks: Vec::new()}
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
    pub fn get_cube_attributes(&self, selected_cube: Option<Vector3>) -> Vec<CubeAttr> {
        let mut positions: Vec<CubeAttr> = Vec::new();
        for chunk in &self.chunks {
            // TODO improve this code
            // I know that this is not the best way to do this:
            // 1. It is not optimal because `.push` is really slow
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

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Destroy { at } => self.destroy_cube(at)
        }
    }

    fn destroy_cube(&mut self, at: Vector3) {
        for chunk in &mut self.chunks {
            if chunk.is_in(&at) {
                chunk.destroy_cube(at);
                return;
            }
        }
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
