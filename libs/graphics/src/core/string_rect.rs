use primitives::font::GLChar;
use crate::renderer::RectRenderData;

/// Represent a string with rectangles
pub struct StringRect {
    rects: Vec<RectRenderData>,
}

impl StringRect {
    pub fn new(string: &String, u: f32, v: f32, size: f32) -> Self {
        let mut rects = Vec::new();
        StringRect::write_string(u, v, size, string, &mut rects);
        Self { rects }
    }

    pub fn rects(&self) -> &Vec<RectRenderData> {
        &self.rects
    }

    /// Transform each character of a String into a RectRenderData and add them to a Vec
    /// Return the u position of the last character
    pub fn write_string(u: f32, v: f32, w: f32, st: &String, rects: &mut Vec<RectRenderData>) -> f32 {
        // This function could probably be moved somewhere else
        for (i, c) in st.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            let gl_char = GLChar::from_char(c);
            rects.push(RectRenderData {
                u: u + i as f32 * w * 3.,
                v,
                w,
                h: w,
                color: primitives::color::Color::Transparent,
                is_font: true,
                font_coords: Some(gl_char.get_webgl_altas_coordinate()),
                block_id: None,
            });
        }
        u + st.len() as f32 * w * 3.
    }

    pub fn write_string_centered(
        v: f32,
        w: f32,
        st: &String,
        rects: &mut Vec<RectRenderData>,
    ) -> f32 {
        StringRect::write_string(-3. * w * (st.len() as f32 - 1.) / 2., v, w, st, rects)
    }
}
