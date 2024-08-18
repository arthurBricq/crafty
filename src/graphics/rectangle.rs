use glium::implement_vertex;
use crate::graphics::color::Color;

pub const RECT_VERTEX_SHADER: &str = r"
    #version 330 core
    in vec3 position;
    in mat4 transformation;
    in vec4 color;

    out vec4 color_s;

    void main()
    {
       gl_Position = transformation * vec4(position, 1.0);
       color_s = color;
    }
    ";

pub const RECT_FRAGMENT_SHADER: &str = r"
    #version 330 core
    in vec4 color_s;
    out vec4 FragColor;
    void main()
    {
        FragColor = color_s;
    }
    ";

#[derive(Copy, Clone)]
pub struct RectVertex {
    /// Position of the pixel on the NDC
    position: [f32; 3],
}

implement_vertex!(RectVertex, position);

pub const RECT_VERTICES: [RectVertex; 6] = [
    RectVertex { position: [1.0, 1.0, 0.] },
    RectVertex { position: [1.0, -1.0, 0.] },
    RectVertex { position: [-1.0, 1.0, 0.] },
    RectVertex { position: [1.0, -1.0, 0.] },
    RectVertex { position: [-1.0, -1.0, 0.] },
    RectVertex { position: [-1.0, 1.0, 0.] },
];


/// Holds the model of 1 tile
#[derive(Copy, Clone)]
pub struct RectVertexAttr {
    transformation: [[f32; 4]; 4],
    /// RGBa color
    color: [f32; 4]
}

implement_vertex!(RectVertexAttr, transformation, color);

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
            color: c.rgba()
        }
    }
    
    pub fn new_with_char(u: f32, v: f32, w: f32, c: char, c: Color) -> Self {
        // TODO arthur
        Self {
            transformation: [
                [  w, 0.0, 0.0, 0.0],
                [0.0,   w, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [  u,   v, 0.0, 1.0]
            ],
            color: c.rgba()
        }
    }
}
