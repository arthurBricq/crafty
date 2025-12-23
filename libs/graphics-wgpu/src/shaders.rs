// WGSL shaders converted from GLSL

pub const CUBE_VERTEX_SHADER: &str = r#"
struct CameraUniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(0) @binding(1) var block_textures: texture_2d_array<f32>;
@group(0) @binding(2) var block_textures_sampler: sampler;
@group(0) @binding(3) var selected_texture: texture_2d<f32>;
@group(0) @binding(4) var selected_texture_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) face: u32,
}

struct InstanceInput {
    @location(3) world_matrix_0: vec4<f32>,
    @location(4) world_matrix_1: vec4<f32>,
    @location(5) world_matrix_2: vec4<f32>,
    @location(6) world_matrix_3: vec4<f32>,
    @location(7) block_id: u32,
    @location(8) is_selected: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) face: u32,
    @location(2) block_id: u32,
    @location(3) is_selected: u32,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let world_matrix = mat4x4<f32>(
        instance.world_matrix_0,
        instance.world_matrix_1,
        instance.world_matrix_2,
        instance.world_matrix_3,
    );
    var out: VertexOutput;
    out.clip_position = camera.projection * camera.view * world_matrix * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    out.face = model.face;
    out.block_id = instance.block_id;
    out.is_selected = instance.is_selected;
    return out;
}
"#;

pub const CUBE_FRAGMENT_SHADER: &str = r#"
struct CameraUniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}

struct FragmentUniforms {
    selected_intensity: f32,
}

@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(0) @binding(1) var block_textures: texture_2d_array<f32>;
@group(0) @binding(2) var block_textures_sampler: sampler;
@group(0) @binding(3) var selected_texture: texture_2d<f32>;
@group(0) @binding(4) var selected_texture_sampler: sampler;
@group(0) @binding(5) var<uniform> fragment_uniforms: FragmentUniforms;

struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
    @location(1) face: u32,
    @location(2) block_id: u32,
    @location(3) is_selected: u32,
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let base_idx = in.block_id * 3u;
    var texture_idx: u32;
    
    if (in.face == 5u) {
        // bottom
        texture_idx = base_idx + 2u;
    } else if (in.face == 4u) {
        // top
        texture_idx = base_idx + 1u;
    } else {
        // sides
        texture_idx = base_idx;
    }
    
    var color = textureSample(block_textures, block_textures_sampler, in.tex_coords, texture_idx);
    
    if (in.is_selected != 0u) {
        let selected_color = textureSample(selected_texture, selected_texture_sampler, in.tex_coords);
        color = mix(color, selected_color, fragment_uniforms.selected_intensity);
    }
    
    return color;
}
"#;

pub const ENTITY_VERTEX_SHADER: &str = r#"
struct CameraUniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniforms;
@group(0) @binding(1) var entity_textures: texture_2d_array<f32>;
@group(0) @binding(2) var entity_textures_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) face: u32,
}

struct InstanceInput {
    @location(3) world_matrix_0: vec4<f32>,
    @location(4) world_matrix_1: vec4<f32>,
    @location(5) world_matrix_2: vec4<f32>,
    @location(6) world_matrix_3: vec4<f32>,
    @location(7) body_part_id: u32,
    @location(8) monster_type: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) face: u32,
    @location(2) body_part_id: u32,
    @location(3) monster_type: u32,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let world_matrix = mat4x4<f32>(
        instance.world_matrix_0,
        instance.world_matrix_1,
        instance.world_matrix_2,
        instance.world_matrix_3,
    );
    var out: VertexOutput;
    out.clip_position = camera.projection * camera.view * world_matrix * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    out.face = model.face;
    out.body_part_id = instance.body_part_id;
    out.monster_type = instance.monster_type;
    return out;
}
"#;

pub const ENTITY_FRAGMENT_SHADER: &str = r#"
@group(0) @binding(1) var entity_textures: texture_2d_array<f32>;
@group(0) @binding(2) var entity_textures_sampler: sampler;

struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
    @location(1) face: u32,
    @location(2) body_part_id: u32,
    @location(3) monster_type: u32,
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let texture_idx = in.face + in.body_part_id * 6u + in.monster_type * 24u;
    return textureSample(entity_textures, entity_textures_sampler, in.tex_coords, texture_idx);
}
"#;

pub const RECT_VERTEX_SHADER: &str = r#"
@group(0) @binding(0) var font_atlas: texture_2d<f32>;
@group(0) @binding(1) var font_atlas_sampler: sampler;
@group(0) @binding(2) var block_textures: texture_2d_array<f32>;
@group(0) @binding(3) var block_textures_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) transformation_0: vec4<f32>,
    @location(3) transformation_1: vec4<f32>,
    @location(4) transformation_2: vec4<f32>,
    @location(5) transformation_3: vec4<f32>,
    @location(6) color: vec4<f32>,
    @location(7) is_font: u32,
    @location(8) font_coords: vec2<f32>,
    @location(9) block_id: i32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) font_coords: vec2<f32>,
    @location(3) is_font: u32,
    @location(4) block_id: i32,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let transformation = mat4x4<f32>(
        instance.transformation_0,
        instance.transformation_1,
        instance.transformation_2,
        instance.transformation_3,
    );
    var out: VertexOutput;
    out.clip_position = transformation * vec4<f32>(model.position, 1.0);
    out.color = instance.color;
    out.tex_coords = model.tex_coords;
    out.font_coords = instance.font_coords;
    out.is_font = instance.is_font;
    out.block_id = instance.block_id;
    return out;
}
"#;

pub const RECT_FRAGMENT_SHADER: &str = r#"
@group(0) @binding(0) var font_atlas: texture_2d<f32>;
@group(0) @binding(1) var font_atlas_sampler: sampler;
@group(0) @binding(2) var block_textures: texture_2d_array<f32>;
@group(0) @binding(3) var block_textures_sampler: sampler;

@group(0) @binding(4) var<uniform> font_offsets: vec2<f32>;

struct FragmentInput {
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) font_coords: vec2<f32>,
    @location(3) is_font: u32,
    @location(4) block_id: i32,
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    if (in.is_font != 0u) {
        let font_uv = in.font_coords + font_offsets * in.tex_coords;
        return textureSample(font_atlas, font_atlas_sampler, font_uv);
    } else if (in.block_id >= 0) {
        let texture_idx = u32(in.block_id) * 3u;
        return textureSample(block_textures, block_textures_sampler, in.tex_coords, texture_idx);
    } else {
        return in.color;
    }
}
"#;
