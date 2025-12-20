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
    pub position: crate::position::Position, // includes rotation
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

impl RectRenderData {
    /// Create a new rectangle
    ///
    /// For the (u,v) coordinates:
    /// * the center is at the center of the screen
    /// * u is looking right
    /// * v is looking up
    /// * Each parameters goes from -1. to +1.
    ///
    /// For the (w,h) length parameters:
    /// * w is over the u coordinates
    /// * h is over the v coordinates
    /// * 1.0 means the entire length of the screen
    /// * 0.0 means it's nothing
    pub fn new(u: f32, v: f32, w: f32, h: f32, color: crate::color::Color) -> Self {
        Self {
            u,
            v,
            w,
            h,
            color,
            is_font: false,
            font_coords: None,
            block_id: None,
        }
    }

    /// Creates a rectangle instance from the bottom left corner of the rectangle
    ///
    /// Takes corner coordinates (u, v) and full dimensions (w, h).
    /// Converts to center coordinates but keeps full dimensions (unlike RectInstance::new_from_corner
    /// which halves dimensions, because RectRenderData stores what RectInstance::new expects).
    pub fn new_from_corner(u: f32, v: f32, w: f32, h: f32, color: crate::color::Color) -> Self {
        Self::new(u + w / 2., v + h / 2., w / 2., h / 2., color)
    }

    /// Creates a square instance from the given corner.
    /// The current aspect ratio of the screen must be provided to properly create the cube
    pub fn square_from_corner(
        u: f32,
        v: f32,
        s: f32,
        aspect_ratio: f32,
        color: crate::color::Color,
    ) -> Self {
        Self::new_from_corner(u, v, s / aspect_ratio, s, color)
    }
}
