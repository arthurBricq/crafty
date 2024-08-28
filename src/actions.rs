use crate::block_kind::Block;
use crate::vector::Vector3;

/// An action is something that will alter the world
#[derive(Debug, PartialEq)]
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
    /// Returns the position where to add a new cube, given the position of the cube that is touched.
    pub fn position_to_generate_cube(touched_cube: &Vector3) -> Vector3 {
        
        let cube = touched_cube.to_cube_coordinates();
        let cube_center = cube + Vector3::new(0.5, 0.5, 0.5);
        let diff = touched_cube - &cube_center;
        
        // Find the coordinates with maximal absolute value 
        // This corresponds to the dot product with the face's normal that is the most aligned with the 
        // touched position.
        // And the sign then provides us with the face where to put the new cube
        
        match (diff[0], diff[1], diff[2]) {
            (x,y,z) if x.abs() >= y.abs() && x.abs() >= z.abs() => {
                if x > 0. {
                    // front
                    cube + Vector3::unit_x()
                } else {
                    cube - Vector3::unit_x()
                }
            }
            (x,y,z) if y.abs() >= x.abs() && y.abs() >= z.abs() => {
                if y > 0. {
                    // top
                    cube + Vector3::unit_y()
                } else {
                    cube - Vector3::unit_y()
                }
            }
            (x,y,z) if z.abs() >= x.abs() && z.abs() >= y.abs() => {
                if z > 0. {
                    // top
                    cube + Vector3::unit_z()
                } else {
                    cube - Vector3::unit_z()
                }
            }
            _ => panic!("Not sure why you have arrived here... diff = {diff:?}"),
        }
    }
}