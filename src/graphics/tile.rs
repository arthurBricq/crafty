use crate::graphics::color::Color::LightGray;
use crate::graphics::font::GLChar;
use crate::graphics::rectangle::RectVertexAttr;

/// A tile is a rectangle drawn on the screen, such as a menu.
pub struct HUDManager {
    /// List of the tiles to be presented on the screen
    rects: Vec<RectVertexAttr>
}

impl HUDManager {
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
    
    pub fn add_crafty_label(&mut self) {
        let h = 0.4;
        let s = 0.05;
        let x0 = -0.3;
        self.rects.push(RectVertexAttr::new_with_char(x0, h, s, GLChar::C));
        self.rects.push(RectVertexAttr::new_with_char(x0 + 1. * s * 3., h, s, GLChar::R));
        self.rects.push(RectVertexAttr::new_with_char(x0 + 2. * s * 3., h, s, GLChar::A));
        self.rects.push(RectVertexAttr::new_with_char(x0 + 3. * s * 3., h, s, GLChar::F));
        self.rects.push(RectVertexAttr::new_with_char(x0 + 4. * s * 3., h, s, GLChar::T));
        self.rects.push(RectVertexAttr::new_with_char(x0 + 5. * s * 3., h, s, GLChar::Y));
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }
}