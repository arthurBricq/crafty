extern crate glium;
extern crate winit;

use glium::{Surface, uniform};
use winit::event::ElementState::Pressed;
use winit::event::RawKeyEvent;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::camera::{Camera};
use crate::cube::VERTICES;
use crate::world::World;

/// The struct in charge of drawing the world
pub struct WorldRenderer {
    camera: Camera,
    world: World
}

impl WorldRenderer {

    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            world: World::new()
        }
    }

    pub fn run(&mut self) {
        // We start by creating the EventLoop, this can only be done once per process.
        // This also needs to happen on the main thread to make the program portable.
        let event_loop = winit::event_loop::EventLoopBuilder::new().build()
            .expect("event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Crafty")
            .build(&event_loop);

        // VBO
        let vertex_buffer = glium::VertexBuffer::new(&display, &VERTICES).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Vertex shader
        // Most basic example with a camera
        let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            gl_Position = perspective * view * model * vec4(position, 1.0);
            v_tex_coords = tex_coords;
        }
    "#;

        // Fragment shader
        let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;
        uniform sampler2D tex2;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

        // Load images
        let image = image::load(std::io::Cursor::new(&include_bytes!("/home/arthur/dev/rust/crafty/resources/awesomeface.png")),
                                image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(&display, image).unwrap();

        // Build the shader program
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        // Start rendering by creating a new frame
        let mut target = display.draw();

        // Which we fill with an opaque blue color
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();

        // By finishing the frame swap buffers and thereby make it visible on the window
        target.finish().unwrap();

        // Now we wait until the program is closed
        event_loop.run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::RedrawRequested => {
                        let mut target = display.draw();
                        // target.clear_color(0.0, 0.0, 1.0, 1.0);
                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                        let perspective = {
                            let (width, height) = target.get_dimensions();
                            let aspect_ratio = height as f32 / width as f32;
                            let fov: f32 = 3.141592 / 3.0;
                            let zfar = 1024.0;
                            let znear = 0.1;
                            let f = 1.0 / (fov / 2.0).tan();
                            [
                                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                                [         0.0         ,     f ,              0.0              ,   0.0],
                                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                            ]
                        };

                        // Configure the GPU to do Depth testing (with a depth buffer)
                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                .. Default::default()
                            },
                            .. Default::default()
                        };

                        for cube in self.world.cubes() {
                            // Define our uniforms
                            let uniforms = uniform! {
                                model: cube.model_matrix(),
                                view: self.camera.view_matrix(),
                                perspective: perspective
                            };

                            target.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
                        }

                        target.finish().unwrap();
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                winit::event::Event::DeviceEvent { event, ..} => match  event {
                    winit::event::DeviceEvent::Key(key) => self.handle_input(key),
                    _ => {}
                }
                _ => (),
            };
        }).unwrap();
    }
}

impl WorldRenderer {
    fn handle_input(&mut self, event: RawKeyEvent) {
        println!("key tapped: {event:?}");
        if (event.state == Pressed) {
            match event.physical_key {
                PhysicalKey::Code(key) => match key {
                    KeyCode::Digit0 => {}
                    KeyCode::Digit1 => {}
                    KeyCode::Digit2 => {}
                    KeyCode::Digit3 => {}
                    KeyCode::Digit4 => {}
                    KeyCode::Digit5 => {}
                    KeyCode::Digit6 => {}
                    KeyCode::Digit7 => {}
                    KeyCode::Digit8 => {}
                    KeyCode::Digit9 => {}
                    KeyCode::KeyW => self.camera.forward(),
                    KeyCode::KeyS => self.camera.backward(),
                    KeyCode::KeyD => self.camera.left(),
                    KeyCode::KeyA => self.camera.right(),
                    KeyCode::KeyJ => self.camera.down(),
                    KeyCode::KeyK => self.camera.up(),
                    _ => {}
                },
                PhysicalKey::Unidentified(_) => {}
            }
        }
    }
}