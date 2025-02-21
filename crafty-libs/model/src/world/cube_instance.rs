use glium::implement_vertex;
use crate::primitives::vector::Vector3;
use crate::world::cube::Cube;

/// An OpenGL type that contains the information for OpenGL's instancing
#[derive(Copy, Clone)]
pub struct CubeInstance {
    world_matrix: [[f32; 4]; 4],
    block_id: u8,
    /// We use an integer, since booleans are not supported
    is_selected: u8,
    position: Vector3,
}

implement_vertex!(CubeInstance, world_matrix, block_id, is_selected);

impl CubeInstance {
    pub fn new(cube: &Cube) -> Self {
        Self {
            world_matrix: Self::model_matrix(cube.position()),
            block_id: cube.block_id(),
            is_selected: false as u8,
            position: cube.position().clone(),
        }
    }

    /// Creates a new selected cube
    /// This cube will be slightly inflated, which is a hack to greatly optimize performances
    /// Using this trick allows us to not have to update the existing `CubeInstance` selection property,
    /// but instead we just insert one extra cube that is inflated.
    pub fn new_selected(cube: &Cube) -> Self {
        Self {
            world_matrix: Self::model_matrix_inflated(&(cube.position())),
            block_id: cube.block_id(),
            is_selected: true as u8,
            position: cube.position().clone(),
        }
    }

    pub fn empty() -> Self {
        Self {
            world_matrix: [[0.; 4]; 4],
            block_id: 0,
            is_selected: 0,
            position: Vector3::empty(),
        }
    }

    pub fn position(&self) -> [f32; 3] {
        self.position.as_array()
    }

    pub fn set_is_selected(&mut self, is_selected: bool) {
        self.is_selected = is_selected as u8;
    }

    pub fn model_matrix(position: &Vector3) -> [[f32; 4]; 4] {
        // TODO As you can see, I added 0.5 at each cube model
        //      It's because I was lazy to edit all the values in `VERTICES` of +0.5, but
        //      it would be nice to do it eventually :)
        [
            [1.00, 0.0, 0.0, 0.0],
            [0.0, 1.00, 0.0, 0.0],
            [0.0, 0.0, 1.00, 0.0],
            [
                position[0] + 0.5,
                position[1] + 0.5,
                position[2] + 0.5,
                1.0f32,
            ],
        ]
    }

    pub fn model_matrix_inflated(position: &Vector3) -> [[f32; 4]; 4] {
        [
            [1.01, 0.0, 0.0, 0.0],
            [0.0, 1.01, 0.0, 0.0],
            [0.0, 0.0, 1.01, 0.0],
            [
                position[0] + 0.5,
                position[1] + 0.5,
                position[2] + 0.5,
                1.0f32,
            ],
        ]
    }
}
