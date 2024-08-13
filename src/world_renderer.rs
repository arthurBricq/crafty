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
use crate::cube::Block;
use crate::graphics::cube::{CUBE_FRAGMENT_SHADER, CUBE_VERTEX_SHADER, VERTICES};
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
        let cube_vertex_buffer = glium::VertexBuffer::new(&display, &VERTICES).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Build the texture library, and change the sampler to use the proper filters
        let textures = self.build_textures_array(&display);
        let samplers = textures.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest);

        // Build the shader program
        let program = glium::Program::from_source(&display, CUBE_VERTEX_SHADER, CUBE_FRAGMENT_SHADER, None).unwrap();

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
                            perspective: self.cam.perspective_matrix(target.get_dimensions()),
                            textures: samplers
                        };

                        // We use OpenGL's instancing feature which allows us to render huge amounts of
                        // cubes at once.
                        let positions = self.world.get_cube_attributes();
                        let position_buffer = glium::VertexBuffer::dynamic(&display, &positions).unwrap();
                        target.draw(
                            (&cube_vertex_buffer, position_buffer.per_instance().unwrap()),
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