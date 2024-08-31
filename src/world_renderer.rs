extern crate glium;
extern crate winit;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::actions::Action;
use crate::actions::Action::{Add, Destroy};
use crate::block_kind::Block;
use crate::block_kind::Block::COBBELSTONE;
use crate::camera::{Camera, MotionState};
use crate::fps::FpsManager;
use crate::graphics::cube::{CUBE_FRAGMENT_SHADER, CUBE_VERTEX_SHADER, VERTICES};
use crate::graphics::font::GLChar;
use crate::graphics::hud_renderer::HUDRenderer;
use crate::graphics::menu_debug::DebugData;
use crate::graphics::rectangle::{RECT_FRAGMENT_SHADER, RECT_VERTEX_SHADER, RECT_VERTICES};
use crate::network::server_update::ServerUpdate;
use crate::player_items::PlayerItems;
use crate::network::proxy::Proxy;
use crate::world::World;
use glium::glutin::surface::WindowSurface;
use glium::texture::Texture2dArray;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{uniform, Display, Surface, Texture2d};
use winit::event::ElementState::{Pressed, Released};
use winit::event::{AxisId, ButtonId, ElementState, RawKeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Fullscreen, Window};

const CLICK_TIME_TO_BREAK: f32 = 2.0;

/// The struct in charge of drawing the world
pub struct WorldRenderer {
    proxy: Arc<Mutex<dyn Proxy>>,

    /// Currently displayed world
    world: World,
    /// Position and orientation of the player
    cam: Camera,
    /// Items of the player
    items: PlayerItems,

    hud_renderer: HUDRenderer,
    fps_manager: FpsManager,

    // Logic for when the user is clicking
    // TODO encapsulate that in another struct
    is_left_clicking: bool,
    click_time: f32,

    /// Is the window displayed in fullscreen ?
    fullscreen: bool,
}

impl WorldRenderer {
    pub fn new(proxy: Arc<Mutex<dyn Proxy>>, world: World, cam: Camera) -> Self {
        Self {
            proxy,
            world,
            cam,
            hud_renderer: HUDRenderer::new(),
            fps_manager: FpsManager::new(),
            items: PlayerItems::new(),
            is_left_clicking: false,
            click_time: 0.0,
            fullscreen: false,
        }
    }

    pub fn login(&mut self) {
        self.proxy.lock().unwrap().login();
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

        // Add some damn items
        for _ in 0..50 {
            self.items.collect(COBBELSTONE);
        }

        // Try to lock the mouse to the window, this doen't work for all OS
        let lock_mouse = window.set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked));
        if lock_mouse.is_err() {
            println!("Can't lock")
        }

        // Construct the buffer of vertices (for single objects, we use OpenGL's instancing to multiply them)
        let cube_vertex_buffer = glium::VertexBuffer::new(&display, &VERTICES).unwrap();
        let rect_vertex_buffer = glium::VertexBuffer::new(&display, &RECT_VERTICES).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Build the texture library, and change the sampler to use the proper filters
        let textures = self.build_textures_array(&display);
        let cubes_texture_sampler = textures.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest);

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

        // Last details before running
        self.hud_renderer.set_player_items(self.items.get_current_items());

        // Initially, ask for server updates
        self.proxy.lock().unwrap().send_position_update(self.cam.position().clone());
        thread::sleep(Duration::from_millis(self.proxy.lock().unwrap().loading_delay()));
        self.handle_server_updates();

        // Initialize cube_to_draw, this SHOULD NOT go into handle_server_update as it is call at every loop !
        self.world.set_cubes_to_draw(self.cam.touched_cube());

        // Event loop
        let mut t = Instant::now();
        event_loop.run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::Resized(_) => {
                        self.hud_renderer.set_dimension(display.get_framebuffer_dimensions());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        let mut target = display.draw();
                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                        // Step the camera with the elapsed time
                        let dt = t.elapsed();
                        if self.cam.touched_cube().is_some() && self.is_left_clicking {
                            self.click_time += dt.as_secs_f32();
                            if self.click_time >= CLICK_TIME_TO_BREAK {
                                // Break the cube
                                self.apply_action(Destroy { at: self.cam.touched_cube().unwrap().to_cube_coordinates() });
                                self.is_left_clicking = false;
                                self.click_time = 0.;
                            }
                        }

                        // Step
                        self.fps_manager.step(dt);
                        self.cam.step(dt, &self.world);
                        t = Instant::now();

                        // Server updates
                        self.proxy.lock().unwrap().send_position_update(self.cam.position().clone());
                        self.handle_server_updates();

                        // HUD updates
                        if self.hud_renderer.show_debug() {
                            self.hud_renderer
                                .set_debug(DebugData::new(self.fps_manager.fps(), self.cam.position().clone(), self.cam.rotation(), self.world.number_cubes_rendered()));
                        }

                        // I) Draw the cubes

                        // Configure the GPU to do Depth testing (with a depth buffer)
                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        };

                        // Define our uniforms (same uniforms for all cubes)...
                        let uniforms = uniform! {
                            view: self.cam.view_matrix(),
                            perspective: self.cam.perspective_matrix(target.get_dimensions()),
                            textures: cubes_texture_sampler,
                            selected_texture: &selected_texture,
                            selected_intensity: if self.is_left_clicking {self.click_time / CLICK_TIME_TO_BREAK} else {0.2},
                        };

                        // We use OpenGL's instancing feature which allows us to render huge amounts of
                        // cubes at once.
                        // OpenGL instancing = instead of setting 1000 times different uniforms, you give once 1000 attributes
                        self.world.set_selected_cube(self.cam.touched_cube());
                        let position_buffer = glium::VertexBuffer::immutable(&display, self.world.cube_to_draw()).unwrap();
                        target.draw(
                            (&cube_vertex_buffer, position_buffer.per_instance().unwrap()),
                            &indices,
                            &cube_program,
                            &uniforms,
                            &params).unwrap();

                        // II) Drawn the tiles
                        let rect_uniforms = uniform! {
                            font_atlas: &font_atlas,
                            font_offsets: GLChar::get_offset(),
                            textures: cubes_texture_sampler
                        };

                        // We change the draw parameters here to allow transparency.
                        let draw_parameters = glium::draw_parameters::DrawParameters {
                            blend: glium::draw_parameters::Blend::alpha_blending(),
                            ..glium::draw_parameters::DrawParameters::default()
                        };

                        let rects_buffer = glium::VertexBuffer::dynamic(&display, self.hud_renderer.rects()).unwrap();
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
                    winit::event::DeviceEvent::Button { button, state } => self.handle_button_event(button, state),
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
            }
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
                        KeyCode::F10 => self.world.save_to_file("map.json"),
                        KeyCode::F11 => self.toggle_fullscreen(&window),
                        KeyCode::F3 => self.hud_renderer.toggle_debug_menu(),
                        KeyCode::F12 => self.hud_renderer.toggle_help_menu(),
                        KeyCode::Escape => std::process::exit(1),
                        _ => {}
                    }
                }
                PhysicalKey::Unidentified(_) => {}
            }
        }
    }

    fn apply_action(&mut self, action: Action) {
        // Handle items
        match action {
            Destroy { at } => {
                if let Some(block) = self.world.block_at(&at) {
                    self.items.collect(block);
                }
            }
            Add { at, block } => {
                self.items.consume(block);
            }
        }
        self.hud_renderer.set_player_items(self.items.get_current_items());

        // Handle cubes
        self.world.apply_action(&action);

        // Forward to server
        self.proxy.lock().unwrap().on_new_action(action);
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
        } else if button == 3 && state == Pressed {
            // Right click = add a new cube
            // We know where is the player and we know 
            if let Some(touched) = self.cam.touched_cube() {
                if let Some(block) = self.items.get_current_block() {
                    self.apply_action(Action::Add {
                        at: Action::position_to_generate_cube(&touched),
                        block,
                    });
                }
            }
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

    fn handle_server_updates(&mut self) {
        let updates = self.proxy.lock().unwrap().consume_server_updates();
        for update in updates {
            match update {
                ServerUpdate::LoadChunk(chunk) => self.world.add_chunk(chunk),
                ServerUpdate::Response(_) => {}
                ServerUpdate::SendAction(action) => self.world.apply_action(&action)
            }
        }
    }
}