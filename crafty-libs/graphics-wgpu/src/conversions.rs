use primitives::position::Position;
use primitives::render_data::{CubeRenderData, EntityRenderData, RectRenderData};
use primitives::vector::Vector3;
use crate::vertices::{CubeInstance, EntityInstance, RectInstance};

/// Compute cube model matrix
pub fn cube_model_matrix(position: &Vector3) -> [[f32; 4]; 4] {
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

/// Compute inflated cube model matrix (for selected cubes)
pub fn cube_model_matrix_inflated(position: &Vector3) -> [[f32; 4]; 4] {
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

/// Compute entity model matrix with full rotation (yaw + pitch)
pub fn entity_model_matrix_rot_yx(position: &Position, scale: [f32; 3]) -> [[f32; 4]; 4] {
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

/// Compute entity model matrix with yaw rotation only
pub fn entity_model_matrix_rot_y(position: &Position, scale: [f32; 3]) -> [[f32; 4]; 4] {
    let yaw = position.yaw();
    [
        [scale[0] * yaw.cos(), 0., scale[0] * yaw.sin(), 0.],
        [0., scale[1], 0., 0.],
        [-scale[2] * yaw.sin(), 0., scale[2] * yaw.cos(), 0.],
        [position.x(), position.y(), position.z(), 1.],
    ]
}

/// Convert CubeRenderData to CubeInstance
impl From<CubeRenderData> for CubeInstance {
    fn from(data: CubeRenderData) -> Self {
        let matrix = if data.is_selected {
            cube_model_matrix_inflated(&data.position)
        } else {
            cube_model_matrix(&data.position)
        };
        CubeInstance::from_matrix_and_ids(matrix, data.block_id, data.is_selected)
    }
}

/// Convert EntityRenderData to EntityInstance
impl From<EntityRenderData> for EntityInstance {
    fn from(data: EntityRenderData) -> Self {
        let matrix = match data.body_part_id {
            0 => entity_model_matrix_rot_yx(&data.position, data.scale),
            _ => entity_model_matrix_rot_y(&data.position, data.scale),
        };
        EntityInstance::from_matrix_and_ids(matrix, data.body_part_id, data.monster_type)
    }
}

/// Convert RectRenderData to RectInstance
impl From<RectRenderData> for RectInstance {
    fn from(data: RectRenderData) -> Self {
        let matrix = if data.is_font {
            // Font rendering: square with size w
            [
                [data.w, 0.0, 0.0, 0.0],
                [0.0, data.w, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [data.u, data.v, 0.0, 1.0],
            ]
        } else {
            // Regular rectangle
            [
                [data.w, 0.0, 0.0, 0.0],
                [0.0, data.h, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [data.u, data.v, 0.0, 1.0],
            ]
        };
        
        RectInstance::from_matrix_and_data(
            matrix,
            data.color.rgba(),
            data.is_font,
            data.font_coords.unwrap_or([0.0, 0.0]),
            data.block_id,
        )
    }
}
