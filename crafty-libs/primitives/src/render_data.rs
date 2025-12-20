/// Backend-agnostic representation of a cube to render
#[derive(Clone, Copy)]
pub struct CubeRenderData {
    pub position: crate::vector::Vector3,
    pub block_id: u8,
    pub is_selected: bool,
}

/// Backend-agnostic representation of an entity part to render
#[derive(Clone)]
pub struct EntityRenderData {
    pub position: crate::position::Position,  // includes rotation
    pub body_part_id: u8,
    pub monster_type: u8,
    pub scale: [f32; 3],
}

/// Backend-agnostic representation of a rectangle to render
#[derive(Clone, Copy)]
pub struct RectRenderData {
    pub u: f32,
    pub v: f32,
    pub w: f32,
    pub h: f32,
    pub color: crate::color::Color,
    pub is_font: bool,
    pub font_coords: Option<[f32; 2]>,
    pub block_id: Option<i8>,
}
