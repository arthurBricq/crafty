pub mod camera;
pub mod color;
pub mod face;
pub mod math;
pub mod matrix;
pub mod position;
pub mod vector;

// TODO: These have to be moved away from primtiives
//       Because I don't want to have to depend on glium, but I am blocked by the orphan rule.
// Problem Recap
// graphics crate: Defines traits like Renderer. Should not depend on backend details.
// graphics_glium crate: Implements Renderer using glium. It needs types that implement glium::Vertex.
// Orphan rule issue: You can’t implement a foreign trait (glium::Vertex) for a foreign type
// (your graphics-defined vertex structs) inside graphics_glium without making graphics depend on glium.
pub mod opengl {
    pub mod cube;
    pub mod cube_instance;
    pub mod entity;
    pub mod font;
    pub mod rectangle;
}
