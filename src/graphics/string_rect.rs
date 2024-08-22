use crate::graphics::font::GLChar;
use crate::graphics::rectangle::RectVertexAttr;

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

    /// Add RectVertexAttr from a string to a vector of RectVertexAttr and return the u position of the last character 
    pub fn write_string(u: f32, v: f32, w: f32, st: &String, rects: &mut Vec<RectVertexAttr>) -> f32 {
        // This function could probably be moved somewhere else
        for (i,c) in st.chars().enumerate() {
            if c== ' ' {continue}
            rects.push(RectVertexAttr::new_with_char(u + i as f32 * w * 2., v, w, GLChar::from_char(c)));
        }
        u + st.len() as f32 * w * 2.
    }

}