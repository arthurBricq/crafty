use glium::implement_vertex;

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
        
        uniform sampler2D selected_texture;

        void main() {
            // Each block has 3 types of faces
            int idx = block_id_s * 3;

            if (face_s == 4) {
                // bottom
                color = texture(textures, vec3(v_tex_coords, idx + 2));
            } else if (face_s == 5) {
                // top
                color = texture(textures, vec3(v_tex_coords, idx + 1));
            } else {
                // sides
                color = texture(textures, vec3(v_tex_coords, float(idx)));
            }

            if (is_selected_s != 0) {
                color = mix(color, texture(selected_texture, v_tex_coords), 0.5);
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
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 0 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 0 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 0 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 0 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 0 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 0 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 1 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 1 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 1 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 1 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 1 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 1 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 2 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 2 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 2 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 2 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 2 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 2 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 3 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 3 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 3 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 3 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 3 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 3 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 4 },
    CubeVertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], face: 4 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 4 },
    CubeVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 4 },
    CubeVertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 4 },
    CubeVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 4 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 5 },
    CubeVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 5 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 5 },
    CubeVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 5 },
    CubeVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 0.0], face: 5 },
    CubeVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 5 }
];


/// An OpenGL type that contains the information for OpenGL's instancing
#[derive(Copy, Clone)]
pub struct CubeAttr {
    world_matrix: [[f32; 4]; 4],
    block_id: u8,
    /// We use an integer, since booleans are not supported
    is_selected: u8
}

implement_vertex!(CubeAttr, world_matrix, block_id, is_selected);

impl CubeAttr {
    pub fn new(world_matrix: [[f32; 4]; 4], block_id: u8, is_selected: bool) -> Self {
        Self { world_matrix, block_id, is_selected: is_selected as u8 }
    }
}
