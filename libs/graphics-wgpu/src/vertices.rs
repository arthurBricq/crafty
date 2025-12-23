use bytemuck::{Pod, Zeroable};

/// Vertex for the base cube mesh
#[repr(C, align(4))] // Ensure 4-byte alignment
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CubeVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub face: u32, // Use u32 instead of u8 to avoid padding issues
}

/// Instance data for cube rendering
/// In wgpu, mat4 is represented as 4 vec4 attributes
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CubeInstance {
    pub world_matrix_0: [f32; 4], // First row of matrix
    pub world_matrix_1: [f32; 4], // Second row of matrix
    pub world_matrix_2: [f32; 4], // Third row of matrix
    pub world_matrix_3: [f32; 4], // Fourth row of matrix
    pub block_id: u32,
    pub is_selected: u32,
}

impl CubeInstance {
    pub fn from_matrix_and_ids(matrix: [[f32; 4]; 4], block_id: u8, is_selected: bool) -> Self {
        Self {
            world_matrix_0: matrix[0],
            world_matrix_1: matrix[1],
            world_matrix_2: matrix[2],
            world_matrix_3: matrix[3],
            block_id: block_id as u32,
            is_selected: if is_selected { 1 } else { 0 },
        }
    }
}

/// Instance data for entity rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct EntityInstance {
    pub world_matrix_0: [f32; 4],
    pub world_matrix_1: [f32; 4],
    pub world_matrix_2: [f32; 4],
    pub world_matrix_3: [f32; 4],
    pub body_part_id: u32,
    pub monster_type: u32,
}

impl EntityInstance {
    pub fn from_matrix_and_ids(matrix: [[f32; 4]; 4], body_part_id: u8, monster_type: u8) -> Self {
        Self {
            world_matrix_0: matrix[0],
            world_matrix_1: matrix[1],
            world_matrix_2: matrix[2],
            world_matrix_3: matrix[3],
            body_part_id: body_part_id as u32,
            monster_type: monster_type as u32,
        }
    }
}

/// Vertex for the base rectangle mesh
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RectVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

/// Instance data for rectangle rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RectInstance {
    pub transformation_0: [f32; 4],
    pub transformation_1: [f32; 4],
    pub transformation_2: [f32; 4],
    pub transformation_3: [f32; 4],
    pub color: [f32; 4],
    pub is_font: u32,
    pub font_coords: [f32; 2],
    pub block_id: i32,
}

impl RectInstance {
    pub fn from_matrix_and_data(
        matrix: [[f32; 4]; 4],
        color: [f32; 4],
        is_font: bool,
        font_coords: [f32; 2],
        block_id: Option<i8>,
    ) -> Self {
        Self {
            transformation_0: matrix[0],
            transformation_1: matrix[1],
            transformation_2: matrix[2],
            transformation_3: matrix[3],
            color,
            is_font: if is_font { 1 } else { 0 },
            font_coords,
            block_id: block_id.map(|id| id as i32).unwrap_or(-1),
        }
    }
}

/// Base cube mesh vertices (36 vertices)
/// Texture coordinates are flipped for WebGPU (V coordinate: 1.0 - original_v)
pub const CUBE_VERTICES: [CubeVertex; 36] = [
    // Right side (face 0)
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 0 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], face: 0 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 0.0], face: 0 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 0.0], face: 0 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], face: 0 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 0 },
    // Front (face 1)
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 1 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], face: 1 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 1 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 1 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], face: 1 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 1 },
    // Left side (face 2)
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], face: 2 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [0.0, 1.0], face: 2 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 0.0], face: 2 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 0.0], face: 2 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 2 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], face: 2 },
    // Back (face 3)
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 0.0], face: 3 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [1.0, 0.0], face: 3 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], face: 3 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], face: 3 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 1.0], face: 3 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 0.0], face: 3 },
    // Top (face 4)
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], face: 4 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 4 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 4 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 4 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 4 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], face: 4 },
    // Bottom (face 5)
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 5 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 5 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], face: 5 }, // 0.0 -> 1.0 - 0.0 = 1.0
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], face: 5 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 5 }, // 1.0 -> 1.0 - 1.0 = 0.0
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 5 },
];

/// Base rectangle mesh vertices (6 vertices for a quad)
/// Texture coordinates are flipped for WebGPU (V coordinate: 1.0 - original_v)
pub const RECT_VERTICES: [RectVertex; 6] = [
    RectVertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0] }, // 1.0 -> 1.0 - 1.0 = 0.0
    RectVertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0] }, // 0.0 -> 1.0 - 0.0 = 1.0
    RectVertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0] }, // 1.0 -> 1.0 - 1.0 = 0.0
    RectVertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
    RectVertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
    RectVertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0] }, // 0.0 -> 1.0 - 0.0 = 1.0
];
