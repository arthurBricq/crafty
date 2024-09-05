use glium::implement_vertex;

use crate::cube::Cube;
use crate::primitives::vector::Vector3;

// Vertex shader
// Most basic example with a camera
pub const CUBE_VERTEX_SHADER: &str = r#"
        #version 150

        in vec3 position;
        in mat4 world_matrix;

        // The vertex shader has some passthrough for the fragment shader...

        // Which face of the cube is being passed ?
        in int face;
        flat out int face_s;

        // Index of the block to be used
        in int block_id;
        flat out int block_id_s;

        // Is the cube currently selected
        in int is_selected;
        flat out int is_selected_s;

        // Where is the vertex located on the face ?
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 perspective;
        uniform mat4 view;

        void main() {
            gl_Position = perspective * view * world_matrix * vec4(position, 1.0);
            v_tex_coords = tex_coords;
            face_s = face;
            block_id_s = block_id;
            is_selected_s = is_selected;
        }
    "#;

// Fragment shader
pub const CUBE_FRAGMENT_SHADER: &str = r#"
        #version 140

        // passed-through the vertex shader
        flat in int face_s;
        flat in int block_id_s;
        flat in int is_selected_s;
        in vec2 v_tex_coords;

        out vec4 color ;

        uniform sampler2DArray textures;
        
        // uniforms for the selected block
        uniform sampler2D selected_texture;
        uniform float selected_intensity;

        void main() {
            // Each block has 3 types of faces
            int idx = block_id_s * 3;

            if (face_s == 5) {
                // bottom
                color = texture(textures, vec3(v_tex_coords, idx + 2));
            } else if (face_s == 4) {
                // top
                color = texture(textures, vec3(v_tex_coords, idx + 1));
            } else {
                // sides
                color = texture(textures, vec3(v_tex_coords, float(idx)));
            }

            if (is_selected_s != 0) {
                color = mix(color, texture(selected_texture, v_tex_coords), selected_intensity);
            }
        }
    "#;

/// A vertex of a cube
/// The position is expressed into the OpenGL reference frame
#[derive(Copy, Clone)]
pub struct CubeVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    face: u8,
}

implement_vertex!(CubeVertex, position, tex_coords, face);

pub const VERTICES: [CubeVertex; 36] = [
    
    // Right side
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 0 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 0 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 0 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 0 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 0 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 0 },
    // Front
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 1 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 1 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 1 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 1 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 1 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 1 },
    // Left side
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 2 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 2 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 2 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 2 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 2 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 2 },
    // Back
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 3 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 3 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 3 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 3 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 3 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 3 },
    // Top
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 4 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], face: 4 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 4 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 4 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 4 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 4 },
    //  Bottom
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 5 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 5 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 5 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 5 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], face: 5 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 5 },
];


/// An OpenGL type that contains the information for OpenGL's instancing
#[derive(Copy, Clone)]
pub struct CubeAttr {
    world_matrix: [[f32; 4]; 4],
    block_id: u8,
    /// We use an integer, since booleans are not supported
    is_selected: u8,
    position: Vector3,  
}

implement_vertex!(CubeAttr, world_matrix, block_id, is_selected);

impl CubeAttr {
    pub fn new(cube: &Cube) -> Self {
        Self { 
            world_matrix: Self::model_matrix(cube.position()), 
            block_id: cube.block_id(), 
            is_selected: false as u8, 
            position: cube.position().clone() }
    }

    pub fn empty() -> Self {
        Self {
            world_matrix: [[0.; 4]; 4],
            block_id: 0,
            is_selected: 0,
            position: Vector3::empty()
        }
    }
    
    pub fn position(&self) -> [f32;3] {
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
            [position[0] + 0.5, position[1] + 0.5, position[2] + 0.5, 1.0f32]
        ]
    }
}

const CONTAINER_SIZE: usize = 100000;

/// A class responsible for holding many cubes.
/// The purpose of this class is to not have to re-allocate all the time
pub struct CubeContainer {
    // data: [CubeAttr; CONTAINER_SIZE],
    data: Vec<CubeAttr>,
    current_size: usize
}

impl CubeContainer {
    pub fn new() -> Self {
        Self { 
            data: vec![CubeAttr::empty(); CONTAINER_SIZE], 
            current_size: 0 
        }
    }
    
    pub fn as_slice(&self) -> &[CubeAttr] {
        &self.data[..self.current_size]
    }
    
    pub fn reset(&mut self) {
        self.current_size = 0;
    }
    
    pub fn push(&mut self, cube: CubeAttr) {
        self.data[self.current_size] = cube;
        self.current_size += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::cube::{CubeAttr, CubeContainer};

    #[test]
    fn test_cube_container() {
        let mut container = CubeContainer::new();
        assert_eq!(container.as_slice().len(), 0);
        
        container.push(CubeAttr::empty());
        container.push(CubeAttr::empty());
        assert_eq!(container.as_slice().len(), 2);
        
        container.reset();
        assert_eq!(container.as_slice().len(), 0);
    }
}
