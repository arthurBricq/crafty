use crate::graphics::color::Color::LightGray;
use crate::graphics::rectangle::RectVertexAttr;

/// A tile is a rectangle drawn on the screen, such as a menu.
pub struct TileManager {
    /// List of the tiles to be presented on the screen
    rects: Vec<RectVertexAttr>
}

impl TileManager {
    pub fn new() -> Self {
        Self { rects: Vec::new() }
    }

    /// Adds a cross in the center of the screen
    pub fn add_cross(&mut self) {
        let w = 0.05;
        let s = 0.01;
        self.rects.push(RectVertexAttr::new(0., 0., w / 1.5 , s, LightGray));
        self.rects.push(RectVertexAttr::new(0., 0., s / 2.5, w, LightGray));
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
}