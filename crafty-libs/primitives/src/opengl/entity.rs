use crate::position::Position;
use glium::implement_vertex;

/// An OpenGL type that contains the information for OpenGL's instancing
#[derive(Copy, Clone)]
pub struct EntityCube {
    world_matrix: [[f32; 4]; 4],
    body_part_id: u8,
    monster_type: u8,
}

implement_vertex!(EntityCube, world_matrix, body_part_id, monster_type);

impl EntityCube {
    /// Build a rendered cube center around position (and not around position + (0.5,0.5,0.5) as for CubeAttr !!!!)
    /// The cube is scaled by scale for each direction
    /// A Yaw rotation is applied (first component of rot)
    /// A Pitch rotation is then applied (second component of rot)
    // Maybe implement roll one day ?
    pub fn new(position: &Position, body_part_id: u8, monster_type: u8, scale: [f32; 3]) -> Self {
        Self {
            world_matrix: Self::model_matrix_rot_yx(position, scale),
            // body part_id correspond to the [6*body_part_id,6*body_part_id+5] texture loaded
            body_part_id,
            monster_type,
        }
    }

    /// Build a rendered cube without pitch rotation
    pub fn new_only_yaw(
        position: &Position,
        body_part_id: u8,
        monster_type: u8,
        scale: [f32; 3],
    ) -> Self {
        Self {
            world_matrix: Self::model_matrix_rot_y(position, scale),
            // body part_id correspond to the [6*body_part_id,6*body_part_id+5] texture loaded
            body_part_id,
            monster_type,
        }
    }

    /// Generate a world matrix with a scaling over each direction
    /// a rotation around y then
    /// a rotation around local x then
    /// a translation
    fn model_matrix_rot_yx(position: &Position, scale: [f32; 3]) -> [[f32; 4]; 4] {
        let yaw = position.yaw();
        let pitch = position.pitch();
        [
            [
                scale[0] * yaw.cos() * pitch.cos(),
                scale[0] * pitch.sin(),
                scale[0] * yaw.sin() * pitch.cos(),
                0.,
            ],
            [
                -scale[1] * yaw.cos() * pitch.sin(),
                scale[1] * pitch.cos(),
                -scale[1] * yaw.sin() * pitch.sin(),
                0.,
            ],
            [-scale[2] * yaw.sin(), 0.0, scale[2] * yaw.cos(), 0.],
            [position.x(), position.y(), position.z(), 1.],
        ]
    }

    /// Generate a world matrix with a scaling over each direction
    /// a rotation around y then
    /// a translation
    fn model_matrix_rot_y(position: &Position, scale: [f32; 3]) -> [[f32; 4]; 4] {
        let yaw = position.yaw();
        [
            [scale[0] * yaw.cos(), 0., scale[0] * yaw.sin(), 0.],
            [0., scale[1], 0., 0.],
            [-scale[2] * yaw.sin(), 0., scale[2] * yaw.cos(), 0.],
            [position.x(), position.y(), position.z(), 1.],
        ]
    }
}
