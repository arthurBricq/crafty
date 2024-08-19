use glium::implement_vertex;
use crate::graphics::color::Color;
use crate::graphics::font::GLChar;

pub const RECT_VERTEX_SHADER: &str = r"
    #version 330 core
    // Attributes of each vertex
    in vec3 position;
    in vec2 tex_coords;

    // Atributes of each tile
    in mat4 transformation;
    in vec4 color;
    in int is_font;
    in vec2 font_coords;

    // Outputs of the vertex shader (for the pixel shader)
    out vec4 color_s;
    out vec2 tex_coords_s;
    out vec2 font_coords_s;
    flat out int is_font_s;

    void main()
    {
       gl_Position = transformation * vec4(position, 1.0);
       color_s = color;
       tex_coords_s = tex_coords;
       font_coords_s = font_coords;
       is_font_s = is_font;
    }
    ";

pub const RECT_FRAGMENT_SHADER: &str = r"
    #version 330 core

    // Inputs of the fragment shader
    in vec4 color_s;
    in vec2 tex_coords_s;
    in vec2 font_coords_s;
    flat in int is_font_s;

    uniform sampler2D font_atlas;
    uniform vec2 font_offsets;

    out vec4 FragColor;

    void main()
    {
        if (is_font_s != 0) {
            // There is a font char to be drawn
            // We need to compute the coordinates of the proper character in the font atlas.
            // To do this, we use 
            // * `font_coords_s`: coordinates of the bottom-left corner of the character
            // * `tex_coords_s` : coordinates within the char rect
            // * `font_offsets` : dimensions of each character 
            FragColor = texture(font_atlas, vec2(font_coords_s[0] + font_offsets[0] * tex_coords_s[0], font_coords_s[1] + font_offsets[1] * tex_coords_s[1]));
        } else {
            // If the tile is not a font, then we just use the background color.
            FragColor = color_s;
        }
    }
    ";

#[derive(Copy, Clone)]
pub struct RectVertex {
    /// Position of the pixel on the NDC
    position: [f32; 3],
    tex_coords: [f32; 2]
}

implement_vertex!(RectVertex, position, tex_coords);

pub const RECT_VERTICES: [RectVertex; 6] = [
    RectVertex { position: [ 1.0,  1.0, 0.], tex_coords: [1., 1.] },
    RectVertex { position: [ 1.0, -1.0, 0.], tex_coords: [1., 0.] },
    RectVertex { position: [-1.0,  1.0, 0.], tex_coords: [0., 1.] },
    RectVertex { position: [-1.0,  1.0, 0.], tex_coords: [0., 1.] },
    RectVertex { position: [ 1.0, -1.0, 0.], tex_coords: [1., 0.] },
    RectVertex { position: [-1.0, -1.0, 0.], tex_coords: [0., 0.] },
];


/// Holds the model of 1 tile
#[derive(Copy, Clone)]
pub struct RectVertexAttr {
    transformation: [[f32; 4]; 4],
    /// RGBa color
    color: [f32; 4],
    /// true if the rect contains a font
    is_font: u8,
    /// Coordinates of the font in the texture atlas
    font_coords: [f32; 2],
}

implement_vertex!(RectVertexAttr, transformation, color, is_font, font_coords);

impl RectVertexAttr {
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
    /// * 1.0 means the entire lenght of the screen
    /// * 0.0 means it's nothing
    pub fn new(u: f32, v: f32, w: f32, h: f32, c: Color) -> Self {
        Self {
            transformation: [
                [  w, 0.0, 0.0, 0.0],
                [0.0,   h, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [  u,   v, 0.0, 1.0]
            ],
            color: c.rgba(),
            is_font: false as u8,
            font_coords: [0., 0.]
        }
    }
    
    /// Creates a new rectangle that draws a given character
    pub fn new_with_char(u: f32, v: f32, w: f32, c: GLChar) -> Self {
        Self {
            transformation: [
                [  w, 0.0, 0.0, 0.0],
                [0.0,   w, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [  u,   v, 0.0, 1.0]
            ],
            color: [0.,0.,0.,0.],
            is_font: true as u8,
            font_coords: c.get_index()
        }
    }
}
