use serde::{Deserialize, Serialize};
use crate::block_kind::Block;
use crate::cube::Cube;
use crate::primitives::vector::Vector3;

/// An action is something that will alter the world
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Action {
    /// Destroys a cube of the world
    Destroy {
        at: Vector3
    },

    // Adds a cube
    Add {
        at: Vector3,
        block: Block
    }
}

impl Action {
    /// Returns the position where to add a new cube, given 
    /// `touched_cube`    : ref to the cube being touched
    /// `touched_position`: position on this cube that is being touched
    pub fn position_to_generate_cube(touched_cube: &Cube, camera_pos: Vector3, camera_dir: Vector3) -> Vector3 {
        let faces = touched_cube.faces();
        let mut best_result = (f32::MAX, 0);
        for i in 0..faces.len() {
            if let Some(t) = faces[i].face_intersection(camera_pos, camera_dir) {
                if t < best_result.0 {
                    best_result = (t, i)
                }
            }
        }
        faces[best_result.1].adjacent_cube()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let as_json = serde_json::to_string(self).unwrap();
        as_json.into_bytes()
    }

    pub fn from_str(text: &str) -> Self {
        serde_json::from_str(text).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::Action;
    use crate::block_kind::Block;
    use crate::cube::Cube;
    use crate::primitives::vector::Vector3;

    #[test] 
    fn test_computation_of_new_cube_position() {
        let cube = Cube::new([0., 0., 0.], Block::COBBELSTONE, 0);
        
        assert_eq!(Vector3::new(1., 0., 0.), 
                   Action::position_to_generate_cube(&cube, Vector3::new(3., 0.5, 0.5), Vector3::unit_x().opposite()));
        
        assert_eq!(Vector3::new(0., 0., 1.),
                   Action::position_to_generate_cube(&cube, Vector3::new(0.5, 0.5, 3.5), Vector3::unit_z().opposite()));
        
        
        
        
        
        
    }
}