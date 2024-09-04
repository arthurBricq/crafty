use glium::implement_vertex;

use crate::vector::Vector3;

pub const ENTITY_VERTEX_SHADER: &str = r#"
        #version 150

        in vec3 position;
        in mat4 world_matrix;

        // The vertex shader has some passthrough for the fragment shader...

        // Which face of the cube is being passed ?
        in int face;
        flat out int face_s;

        // Index of the block to be used
        in int body_part_id;
        flat out int body_part_id_s;

        // Where is the vertex located on the face ?
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 perspective;
        uniform mat4 view;

        void main() {
            gl_Position = perspective * view * world_matrix * vec4(position, 1.0);
            v_tex_coords = tex_coords;
            face_s = face;
            body_part_id_s = body_part_id;
        }
    "#;

// Fragment shader
pub const ENTITY_FRAGMENT_SHADER: &str = r#"
        #version 140

        // passed-through the vertex shader
        flat in int face_s;
        flat in int body_part_id_s;
        in vec2 v_tex_coords;

        out vec4 color ;

        uniform sampler2DArray textures;
        
        void main() {
            // Each block has 6 types of faces
            int idx = body_part_id_s * 6;
            color = texture(textures, vec3(v_tex_coords, idx + face_s));
         }
    "#;


/// A vertex of a cube
/// The position is expressed into the OpenGL reference frame
#[derive(Copy, Clone)]
pub struct EntityVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    face: u8,
}

implement_vertex!(EntityVertex, position, tex_coords, face);

/// This is different from CubeVertex, the face numbering have been changed
pub const ENTITY_VERTICES: [EntityVertex; 36] = [
    
    // Right side
    EntityVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 0 },
    EntityVertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 0 },
    EntityVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 0 },
    EntityVertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 0 },
    EntityVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 0 },
    EntityVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 0 },
    // Front
    EntityVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 1 },
    EntityVertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 1 },
    EntityVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 1 },
    EntityVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 1 },
    EntityVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 1 },
    EntityVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 1 },
    // Left side
    EntityVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 2 },
    EntityVertex { position: [0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 2 },
    EntityVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 2 },
    EntityVertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 2 },
    EntityVertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 2 },
    EntityVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 2 },
    // Back
    EntityVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 3 },
    EntityVertex { position: [-0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], face: 3 },
    EntityVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 3 },
    EntityVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], face: 3 },
    EntityVertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], face: 3 },
    EntityVertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], face: 3 },
    // Top
    EntityVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 4 },
    EntityVertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 0.0], face: 4 },
    EntityVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 4 },
    EntityVertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], face: 4 },
    EntityVertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], face: 4 },
    EntityVertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], face: 4 },
    //  Bottom
    EntityVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 5 },
    EntityVertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], face: 5 },
    EntityVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 5 },
    EntityVertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], face: 5 },
    EntityVertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 1.0], face: 5 },
    EntityVertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], face: 5 },
];


/// An OpenGL type that contains the information for OpenGL's instancing
#[derive(Copy, Clone)]
pub struct EntityCube {
    world_matrix: [[f32; 4]; 4],
    body_part_id: u8,
}

implement_vertex!(EntityCube, world_matrix, body_part_id);

impl EntityCube {
    /// Build a rendered cube center around position (and not around position + (0.5,0.5,0.5) as for CubeAttr !!!!)
    /// The cube is scaled by scale for each direction
    /// A Yaw rotation is applied (first component of rot)
    /// A Pitch roation is then apllied (second component of rot)
    // Maybe implement roll one day ?
    pub fn new(position: &Vector3, body_part_id: u8, scale: [f32; 3], rot: [f32; 2]) -> Self {
        Self { 
            world_matrix: Self::model_matrix_rot_yx(position, scale, rot), 
            // body part_id correspond to the [6*body_part_id,6*body_part_id+5] texture loaded
            body_part_id,
        }
    }

    /// Generate a world matrix with a scaing over each direction
    /// a rotation around y then
    /// a rotation around local x
    fn model_matrix_rot_yx(position: &Vector3, scale: [f32; 3], rot: [f32; 2]) -> [[f32; 4]; 4] {
        [
            [  scale[0] * rot[0].cos() * rot[1].cos(), scale[0] * rot[1].sin(),  scale[0] * rot[0].sin() * rot[1].cos(), 0.0],
            [ -scale[1] * rot[0].cos() * rot[1].sin(), scale[1] * rot[1].cos(), -scale[1] * rot[0].sin() * rot[1].sin(), 0.0],
            [ -scale[2] * rot[0].sin()               , 0.0                    ,  scale[2] * rot[0].cos()               , 0.0],
            
            [position[0]                             , position[1]            , position[2]                            , 1.0f32]
        ]
    }
}
