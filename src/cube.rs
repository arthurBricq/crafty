use glium::implement_vertex;
use crate::world_renderer::{DIRT, GRASS_SIDE, GRASS_TOP};

/// A minecraft in the 3D world
pub struct Cube {
    position: [f32; 3],
}

impl Cube {
    pub fn new(position: [f32; 3]) -> Self {
        Self { position }
    }

    pub fn model_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.00, 0.0, 0.0, 0.0],
            [0.0, 1.00, 0.0, 0.0],
            [0.0, 0.0, 1.00, 0.0],
            [self.position[0], self.position[1], self.position[2], 1.0f32]
        ]
    }
    
    pub fn top_texture(&self) -> &str {
        GRASS_TOP
    }
    
    pub fn side_texture(&self) -> &str {
        GRASS_SIDE
    }
    
    pub fn bottom_texture(&self) -> &str {
        DIRT
    }
}

/// A vertex of a cube
/// The position is expressed into the OpenGL reference frame
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    face: u8,
}

implement_vertex!(Vertex, position, tex_coords, face);

pub const VERTICES: [Vertex; 36] = [
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords:  [0.0, 0.0], face: 0},
    Vertex { position: [0.5, -0.5, -0.5], tex_coords:  [1.0, 0.0], face: 0},
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [ 1.0, 1.0], face: 0},
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [ 1.0, 1.0], face: 0},
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [ 0.0, 1.0], face: 0},
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 0.0], face: 0},

    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [ 0.0, 0.0], face: 1},
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [ 1.0, 0.0], face: 1},
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [ 1.0, 1.0], face: 1},
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [ 1.0, 1.0], face: 1},
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [ 0.0, 1.0], face: 1},
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [ 0.0, 0.0], face: 1},

    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [ 1.0, 0.0], face: 2},
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [ 1.0, 1.0], face: 2},
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0], face: 2},
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0], face: 2},
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [ 0.0, 0.0], face: 2},
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [ 1.0, 0.0], face: 2},

    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [ 0.0, 1.0], face: 3},
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [ 1.0, 1.0], face: 3},
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [ 1.0, 0.0], face: 3},
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [ 1.0, 0.0], face: 3},
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [ 0.0, 0.0], face: 3},
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [ 0.0, 1.0], face: 3},

    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0], face: 4},
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [ 1.0, 1.0], face: 4},
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [ 1.0, 0.0], face: 4},
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [ 1.0, 0.0], face: 4},
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [ 0.0, 0.0], face: 4},
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 0.0, 1.0], face: 4},

    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [ 0.0, 1.0], face: 5},
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [ 1.0, 1.0], face: 5},
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [ 1.0, 0.0], face: 5},
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [ 1.0, 0.0], face: 5},
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [ 0.0, 0.0], face: 5},
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [ 0.0, 1.0], face: 5}
];


/// An OpenGL type that contains the information for OpenGL's instancing
#[derive(Copy, Clone)]
pub struct InstanceAttr {
    world_matrix: [[f32; 4]; 4],
}

implement_vertex!(InstanceAttr, world_matrix);

impl InstanceAttr {
    pub fn new(world_matrix: [[f32; 4]; 4]) -> Self {
        Self { world_matrix }
    }
}