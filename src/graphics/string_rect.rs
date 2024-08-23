use crate::graphics::font::GLChar;
use crate::graphics::rectangle::RectVertexAttr;

use crate::graphics::color::Color::{LightCoral,LightGray,Red};

/// Represent a string with rectangles
pub struct StringRect {
    rects: Vec<RectVertexAttr>,
}

impl StringRect {
    pub fn new(string: &String, u:f32, v:f32, size: f32) -> Self {
        let mut rects=Vec::new();
        StringRect::write_string(u, v, size, string, &mut rects);
        Self {
            rects
        }
    }

    pub fn rects(&self) -> &Vec<RectVertexAttr> {
        &self.rects
    }

    /// Transform each character of a String into a RectVertexAttr and add them to a Vec
    /// Return the u position of the last character 
    pub fn write_string(u: f32, v: f32, w: f32, st: &String, rects: &mut Vec<RectVertexAttr>) -> f32 {
        // This function could probably be moved somewhere else
        for (i,c) in st.chars().enumerate() {
            if c== ' ' {continue}
            rects.push(RectVertexAttr::new_with_char(u + i as f32 * w * 3., v, w, GLChar::from_char(c)));
        }
        u + st.len() as f32 * w * 3.
    }

    pub fn write_string_centered(v: f32, w: f32, st: &String, rects: &mut Vec<RectVertexAttr>) -> f32 {
        StringRect::write_string(-3. * w*(st.len() as f32 -1.)/2., v, w, st, rects)
    }

}