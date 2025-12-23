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



