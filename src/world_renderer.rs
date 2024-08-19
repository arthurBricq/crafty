extern crate glium;
extern crate winit;

use std::time::Instant;

use glium::{Display, Surface, Texture2d, uniform};
use glium::glutin::surface::WindowSurface;
use glium::texture::Texture2dArray;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use winit::event::ElementState::{Pressed, Released};
use winit::event::{AxisId, ButtonId, ElementState, RawKeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Fullscreen, Window};
use crate::actions::Action::Destroy;
use crate::camera::{Camera, MotionState};
use crate::cube::Block;
use crate::fps::FpsManager;
use crate::graphics::cube::{CUBE_FRAGMENT_SHADER, CUBE_VERTEX_SHADER, VERTICES};
use crate::graphics::font::GLChar;
use crate::graphics::rectangle::{RECT_FRAGMENT_SHADER, RECT_VERTEX_SHADER, RECT_VERTICES};
use crate::graphics::tile::HUDManager;
use crate::world::World;

const CLICK_TIME_TO_BREAK: f32 = 2.0;

/// The struct in charge of drawing the world
pub struct WorldRenderer {
    world: World,
    cam:   Camera,
    tile_manager: HUDManager,
    fps_manager: FpsManager,
    
    // Logic for when the user is clicking
    // TODO encapsulate that in another struct
    is_left_clicking: bool,
    click_time: f32,

    /// Is the window displayed in fullscreen ?
    fullscreen: bool,
}

impl WorldRenderer {

    pub fn new(world: World, cam: Camera) -> Self {
        Self {
            world,
            cam,
            tile_manager: HUDManager::new(),
            fps_manager: FpsManager::new(),
            is_left_clicking: false,
            click_time: 0.0,
            fullscreen: false
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

        window.set_cursor_visible(false);

        // Construct the buffer of vertices (for single objects, we use OpenGL's instancing to multiply them)
        let cube_vertex_buffer = glium::VertexBuffer::new(&display, &VERTICES).unwrap();
        let rect_vertex_buffer = glium::VertexBuffer::new(&display, &RECT_VERTICES).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Build the texture library, and change the sampler to use the proper filters
        let textures = self.build_textures_array(&display);
        let samplers = textures.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest);

        // Load other textures that are used
        let selected_texture = Self::load_texture(std::fs::read("./resources/selected.png").unwrap().as_slice(), &display);
        let font_atlas = Self::load_texture(std::fs::read("./resources/fonts.png").unwrap().as_slice(), &display);

        // Build the shader programs
        let cube_program = glium::Program::from_source(&display, CUBE_VERTEX_SHADER, CUBE_FRAGMENT_SHADER, None).unwrap();
        let rect_program = glium::Program::from_source(&display, RECT_VERTEX_SHADER, RECT_FRAGMENT_SHADER, None).unwrap();

        // Start rendering by creating a new frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        self.tile_manager.add_cross();
        self.tile_manager.add_crafty_label();
        
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
                        let dt = t.elapsed();
                        if self.cam.selected().is_some() && self.is_left_clicking {
                            self.click_time += dt.as_secs_f32();
                            if self.click_time >= CLICK_TIME_TO_BREAK {
                                // Break the cube
                                self.world.apply_action(Destroy {at: self.cam.selected().unwrap()});
                                self.is_left_clicking = false;
                                self.click_time = 0.;
                            }
                        }

                        // Step
                        self.fps_manager.step(dt);
                        self.cam.step(dt, &self.world);
                        t = Instant::now();

                        // I) Draw the cubes

                        // Define our uniforms (same uniforms for all cubes)...
                        let uniforms = uniform! {
                            view: self.cam.view_matrix(),
                            perspective: self.cam.perspective_matrix(target.get_dimensions()),
                            textures: samplers,
                            selected_texture: &selected_texture,
                            selected_intensity: if self.is_left_clicking {self.click_time / CLICK_TIME_TO_BREAK} else {0.2},
                        };

                        // We use OpenGL's instancing feature which allows us to render huge amounts of
                        // cubes at once.
                        // OpenGL instancing = instead of setting 1000 times different uniforms, you give once 1000 attributes
                        let positions = self.world.get_cube_attributes(self.cam.selected());
                        let position_buffer = glium::VertexBuffer::dynamic(&display, &positions).unwrap();
                        target.draw(
                            (&cube_vertex_buffer, position_buffer.per_instance().unwrap()),
                            &indices,
                            &cube_program,
                            &uniforms,
                            &params).unwrap();

                        // II) Drawn the tiles
                        let rect_uniforms = uniform! {
                            font_atlas: &font_atlas,
                            font_offsets: GLChar::get_offset()
                        };
                        // We change the draw parameters here to allow transparency.
                        let draw_parameters = glium::draw_parameters::DrawParameters {
                            blend: glium::draw_parameters::Blend::alpha_blending(),
                            ..glium::draw_parameters::DrawParameters::default()
                        };
                        let rects_buffer = glium::VertexBuffer::dynamic(&display, self.tile_manager.rects()).unwrap();
                        target.draw(
                            (&rect_vertex_buffer, rects_buffer.per_instance().unwrap()),
                            &indices,
                            &rect_program,
                            &rect_uniforms,
                            &draw_parameters).unwrap();

                        target.finish().unwrap();
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => window.request_redraw(),
                winit::event::Event::DeviceEvent { event, .. } => match event {
                    winit::event::DeviceEvent::Key(key) => self.handle_key_event(key, &window),
                    winit::event::DeviceEvent::Motion { axis, value } => self.handle_motion_event(axis, value),
                    winit::event::DeviceEvent::Button {button, state} => self.handle_button_event(button, state),
                    _ => {}
                }
                _ => (),
            };
        }).unwrap();
    }


    /// Builds the array of 2D textures using all the blocks
    /// Each block is associated with 3 textures: side, top and bottom
    /// All these textures are loaded into one single texture array, that is fed to OpenGL.
    /// The fragment shader responsible for the cubes is then in charge of selecting the correct element of this array.
    fn build_textures_array(&self, display: &Display<WindowSurface>) -> Texture2dArray {
        // Get the path of the block textures
        let root = "./resources/block/";
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

    /// Loads a texture and returns it
    fn load_texture(bytes: &[u8], display: &Display<WindowSurface>) -> Texture2d {
        let image = image::load(std::io::Cursor::new(bytes),
                                image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        Texture2d::new(display, image).unwrap()
    }

    fn handle_key_event(&mut self, event: RawKeyEvent, window: &Window) {
        // Handle keys related to motion (toggle is important here)
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

        // Second match is for other stuff that only needs to be detected when pressed
        if event.state == Pressed {
            match event.physical_key {
                PhysicalKey::Code(key) => {
                    match key {
                        KeyCode::Digit0 => {}
                        KeyCode::KeyP => {
                            println!("=================");
                            println!("Debug Information");
                            println!("=================");
                            self.cam.debug();
                        }
                        KeyCode::F11 => self.toggle_fullscreen(&window),
                        KeyCode::Escape => std::process::exit(1),
                        _ => {}
                    }
                },
                PhysicalKey::Unidentified(_) => {}
            }
        }
    }

    fn handle_button_event(&mut self, button: ButtonId, state: ElementState) {
        if button == 1 {
            // Left click
            if !self.is_left_clicking && state == Pressed {
                self.is_left_clicking = true;
            } else if self.is_left_clicking && state == Released {
                self.is_left_clicking = false;
                self.click_time = 0.;
            }
        } else if button == 3 {
            // Right click
            // TODO place a cube
        }
    }

    fn handle_motion_event(&mut self, axis: AxisId, value: f64) {
        if axis == 0 {
            self.cam.mousemove(value as f32, 0.0, 0.005);
        } else {
            self.cam.mousemove(0.0, -value as f32, 0.005);
        }
    }

    fn toggle_fullscreen(&mut self, window: &Window) {
        if self.fullscreen {
            window.set_fullscreen(None);
            self.fullscreen = false;
        } else {
            let monitor_handle = window.available_monitors().next().unwrap();
            window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor_handle))));
            self.fullscreen = true;
        }
    }

}