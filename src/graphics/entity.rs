use glium::implement_vertex;
use crate::primitives::position::Position;


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

        in int monster_type;
        flat out int monster_type_s;

        uniform mat4 perspective;
        uniform mat4 view;

        void main() {
            gl_Position = perspective * view * world_matrix * vec4(position, 1.0);
            v_tex_coords = tex_coords;
            face_s = face;
            body_part_id_s = body_part_id;
            monster_type_s = monster_type;
        }
    "#;

// Fragment shader
pub const ENTITY_FRAGMENT_SHADER: &str = r#"
        #version 140

        // passed-through the vertex shader
        flat in int face_s;
        flat in int body_part_id_s;
        flat in int monster_type_s;
        in vec2 v_tex_coords;

        out vec4 color ;

        uniform sampler2DArray entity_textures;
        
        void main() {
            // Each block has 6 types of faces
            // There is 4 different block building a monster
            int idx = face_s + body_part_id_s * 6 + monster_type_s * 4 * 6;
            color = texture(entity_textures, vec3(v_tex_coords, idx));
         }
    "#;

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
    /// A Pitch roation is then apllied (second component of rot)
    // Maybe implement roll one day ?
    pub fn new(position: &Position, body_part_id: u8, monster_type: u8, scale: [f32; 3]) -> Self {
        Self { 
            world_matrix: Self::model_matrix_rot_yx(position, scale),
            // body part_id correspond to the [6*body_part_id,6*body_part_id+5] texture loaded
            body_part_id,
            monster_type
        }
    }

    /// Build a rendered cube without pitch rotation
    pub fn new_only_yaw(position: &Position, body_part_id: u8, monster_type: u8, scale: [f32; 3]) -> Self {
        Self { 
            world_matrix: Self::model_matrix_rot_y(position, scale),
            // body part_id correspond to the [6*body_part_id,6*body_part_id+5] texture loaded
            body_part_id,
            monster_type
        }
    }

    /// Generate a world matrix with a scaing over each direction
    /// a rotation around y then
    /// a rotation around local x then
    /// a translation
    fn model_matrix_rot_yx(position: &Position, scale: [f32; 3]) -> [[f32; 4]; 4] {
        let yaw = position.yaw();
        let pitch = position.pitch();
        [
            [  scale[0] * yaw.cos() * pitch.cos(), scale[0] *pitch.sin(),  scale[0] * yaw.sin() * pitch.cos(), 0.],
            [ -scale[1] * yaw.cos() * pitch.sin(), scale[1] *pitch.cos(), -scale[1] * yaw.sin() * pitch.sin(), 0.],
            [               -scale[2] * yaw.sin(),                   0.0,                scale[2] * yaw.cos(), 0.],
            [                        position.x(),          position.y(),                        position.z(), 1.]
        ]
    }

    /// Generate a world matrix with a scaing over each direction
    /// a rotation around y then
    /// a translation
    fn model_matrix_rot_y(position: &Position, scale: [f32; 3]) -> [[f32; 4]; 4] {
        let yaw = position.yaw();
        [
            [  scale[0] * yaw.cos(),           0.,  scale[0] * yaw.sin(), 0.],
            [                    0.,     scale[1],                    0., 0.],
            [ -scale[2] * yaw.sin(),           0.,  scale[2] * yaw.cos(), 0.],
            [          position.x(), position.y(),          position.z(), 1.]
        ]
    }
}
