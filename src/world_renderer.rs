extern crate glium;
extern crate winit;

use std::time::Instant;

use glium::{Display, Surface, uniform};
use glium::glutin::surface::WindowSurface;
use glium::texture::Texture2dArray;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use winit::event::ElementState::Pressed;
use winit::event::RawKeyEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::camera::{Camera, MotionState};
use crate::cube::{Block, VERTICES};
use crate::world::World;

/// The struct in charge of drawing the world
pub struct WorldRenderer<'a> {
    world: &'a World,
    cam: &'a mut Camera<'a>,
}

impl<'a> WorldRenderer<'a> {

    pub fn new(world: &'a World, cam: &'a mut Camera<'a>) -> Self {
        Self { world, cam }
    }

    pub fn run(&mut self) {
        // We start by creating the EventLoop, this can only be done once per process.
        // This also needs to happen on the main thread to make the program portable.
        let event_loop = winit::event_loop::EventLoopBuilder::new().build()
            .expect("event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Crafty")
            .build(&event_loop);

        window.set_cursor_visible(false);

        // VBO
        let vertex_buffer = glium::VertexBuffer::new(&display, &VERTICES).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Vertex shader
        // Most basic example with a camera
        let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in mat4 world_matrix;

        // The vertex shader has some passthrough for the fragment shader...

        // Which face of the cube is being passed ?
        in int face;
        flat out int face_s;

        // Index of the block to be used
        in int block_id;
        flat out int block_id_s;

        // Where is the vertex located on the face ?
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 perspective;
        uniform mat4 view;

        void main() {
            gl_Position = perspective * view * world_matrix * vec4(position, 1.0);
            v_tex_coords = tex_coords;
            face_s = face;
            block_id_s = block_id;
        }
    "#;

        // Fragment shader
        let fragment_shader_src = r#"
        #version 140

        // passed-through the vertex shader
        flat in int face_s;
        flat in int block_id_s;
        in vec2 v_tex_coords;

        out vec4 color ;

        uniform sampler2DArray textures;

        void main() {
            // Each block has 3 types of faces
            int idx = block_id_s * 3;

            if (face_s == 4) {
                // bottom
                color = texture(textures, vec3(v_tex_coords, idx + 2));
            } else if (face_s == 5) {
                // top
                color = texture(textures, vec3(v_tex_coords, idx + 1));
            } else {
                // sides
                color = texture(textures, vec3(v_tex_coords, float(idx)));
            }
        }
    "#;

        // Build the texture library, and change the sampler to use the proper filters
        let textures = self.build_textures_array(&display);
        let samplers = textures.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest);

        // Build the shader program
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        // Start rendering by creating a new frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
        
        // Event loop 
        let mut t = Instant::now();
        event_loop.run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::RedrawRequested => {
                        let mut target = display.draw();
                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                        let perspective = {
                            let (width, height) = target.get_dimensions();
                            let aspect_ratio = height as f32 / width as f32;
                            let fov: f32 = std::f32::consts::PI / 3.0;
                            let zfar = 1024.0;
                            let znear = 0.1;
                            let f = 1.0 / (fov / 2.0).tan();
                            [
                                [f * aspect_ratio, 0.0, 0.0, 0.0],
                                [0.0, f, 0.0, 0.0],
                                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                            ]
                        };

                        // Configure the GPU to do Depth testing (with a depth buffer)
                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        };
                        
                        // Step the camera with the elapsed time
                        self.cam.step(t.elapsed());
                        t = Instant::now();

                        // Define our uniforms (same uniforms for all cubes)...
                        let uniforms = uniform! {
                            view: self.cam.view_matrix(),
                            perspective: perspective,
                            textures: samplers
                        };

                        // We use OpenGL's instancing feature which allows us to render huge amounts of
                        // cubes at once.
                        let positions = self.world.get_cube_attributes();
                        let position_buffer = glium::VertexBuffer::dynamic(&display, &positions).unwrap();
                        target.draw(
                            (&vertex_buffer, position_buffer.per_instance().unwrap()),
                            &indices,
                            &program,
                            &uniforms,
                            &params).unwrap();

                        target.finish().unwrap();
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                winit::event::Event::DeviceEvent { event, .. } => match event {
                    winit::event::DeviceEvent::Key(key) => self.handle_input(key),
                    winit::event::DeviceEvent::Motion { axis, value } => {
                        if axis == 0 {
                            self.cam.mousemove(value as f32, 0.0, 0.005);
                        } else {
                            self.cam.mousemove(0.0, -value as f32, 0.005);
                        }
                    }
                    _ => {}
                }
                _ => (),
            };
        }).unwrap();
    }

    /// Builds the array of 2D textures using all the blocks
    fn build_textures_array(&self, display: &Display<WindowSurface>) -> Texture2dArray {
        let root = "/home/arthur/dev/rust/crafty/resources/block/";
        let extension = ".png";
        let all_textures = Block::get_texture_files();
        let source = all_textures.iter().map(|name| {
            println!(" Adding texture {name} into texture array");
            let data = std::fs::read(root.to_string() + name + extension).unwrap();
            let image = image::load(std::io::Cursor::new(data), image::ImageFormat::Png).unwrap().to_rgba8();
            let image_dimensions = image.dimensions();
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
        }).collect();
        Texture2dArray::new(display, source).unwrap()
    }

    fn handle_input(&mut self, event: RawKeyEvent) {
        println!("key tapped: {event:?}");
        
        match event.physical_key {
            PhysicalKey::Code(key) => {
                match key {
                    KeyCode::KeyW => self.cam.toggle_state(MotionState::W),
                    KeyCode::KeyS => self.cam.toggle_state(MotionState::S),
                    KeyCode::KeyD => self.cam.toggle_state(MotionState::D),
                    KeyCode::KeyA => self.cam.toggle_state(MotionState::A),
                    KeyCode::KeyK => self.cam.up(),
                    KeyCode::KeyJ => self.cam.down(),
                    KeyCode::Space => self.cam.jump(),
                    _ => {}
                }
            },
            _ => {}
        }

        // Second match is for other stuff...
        if event.state == Pressed {
            match event.physical_key {
                PhysicalKey::Code(key) => {
                    match key {
                        KeyCode::Digit0 => {}
                        KeyCode::Digit1 => {}
                        _ => {}
                    }
                },
                PhysicalKey::Unidentified(_) => {}
            }
        }
    }

}