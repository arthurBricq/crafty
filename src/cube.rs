use glium::implement_vertex;

use strum::IntoEnumIterator;
use strum::EnumIter;
// use strum_macros::EnumIter;

/// The kind of cube
/// Each kind is associated with 3 textures: side, top & bottom.
#[derive(Clone, Copy, EnumIter, PartialEq)]
pub enum Block {
    GRASS = 0,
    DIRT
}

impl Block {
    fn file_name(&self) -> String {
        match self {
            Block::GRASS => "grass_block".to_string(),
            Block::DIRT => "dirt".to_string()
        }
    }
    
    /// Returns a list of all the textures to be loaded, in the proper order.
    pub fn get_texture_files() -> Vec<String>{
        let mut names = Vec::new();
        for block_kind in Block::iter() {
            let name = block_kind.file_name();
            names.push(name.clone() + "_side");
            names.push(name.clone() + "_top");
            names.push(name.clone() + "_bottom");
        }
        names
    }
}

/// Model of a cube in the 3D world.
#[derive(Clone, Copy)]
pub struct Cube {
    position: [f32; 3],
    block: Block
}


impl Cube {
    pub fn new(position: [f32; 3], block: Block) -> Self {
        Self { position, block}
    }

    pub fn model_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.00, 0.0, 0.0, 0.0],
            [0.0, 1.00, 0.0, 0.0],
            [0.0, 0.0, 1.00, 0.0],
            [self.position[0], self.position[1], self.position[2], 1.0f32]
        ]
    }

    pub fn block_id(&self) -> u8 {
        self.block as u8
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

    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [ 0.0, 1.0], face: 2},
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [ 1.0, 1.0], face: 2},
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 1.0, 0.0], face: 2},
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [ 1.0, 0.0], face: 2},
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [ 0.0, 0.0], face: 2},
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [ 0.0, 1.0], face: 2},

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
    block_id: u8
}

implement_vertex!(InstanceAttr, world_matrix, block_id);

impl InstanceAttr {
    pub fn new(world_matrix: [[f32; 4]; 4], block_id: u8) -> Self {
        Self { world_matrix, block_id }
    }
}