extern crate glium;
extern crate winit;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::actions::Action;
use crate::actions::Action::{Add, Destroy};
use crate::block_kind::Block::{COBBELSTONE, DIRT, GRASS, OAKLEAVES, OAKLOG};
use crate::camera::{Camera, MotionState};
use crate::entity::entity_manager::EntityManager;
use crate::entity::humanoid;
use crate::fps::FpsManager;
use crate::graphics::cube::{CUBE_FRAGMENT_SHADER, CUBE_VERTEX_SHADER, VERTICES};

use crate::graphics::entity::{ENTITY_FRAGMENT_SHADER, ENTITY_VERTEX_SHADER};
use crate::graphics::font::GLChar;
use crate::graphics::hud_renderer::HUDRenderer;
use crate::graphics::menu_debug::DebugData;
use crate::graphics::rectangle::{RECT_FRAGMENT_SHADER, RECT_VERTEX_SHADER, RECT_VERTICES};
use crate::network::proxy::Proxy;
use crate::network::server_update::ServerUpdate;
use crate::player_items::PlayerItems;
use crate::texture;
use crate::world::World;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{uniform, Surface};
use winit::event::ElementState::{Pressed, Released};
use winit::event::{AxisId, ButtonId, ElementState, RawKeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Fullscreen, Window};
use crate::graphics::color::Color;

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

    entity_manager: EntityManager,

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
            entity_manager: EntityManager::new(),
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
            self.items.collect(OAKLEAVES);
            self.items.collect(DIRT);
            self.items.collect(GRASS);
            self.items.collect(OAKLOG);
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
        let textures = texture::build_textures_array(&display);
        let cubes_texture_sampler = textures.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest);

        // Load other textures that are used
        let selected_texture = texture::load_texture(std::fs::read("./resources/selected.png").unwrap().as_slice(), &display);
        let font_atlas = texture::load_texture(std::fs::read("./resources/fonts.png").unwrap().as_slice(), &display);

        // Textures for the player
        let player_texture = humanoid::load_humanoid_textures(std::fs::read("./resources/entity/player.png").unwrap().as_slice(), &display);
        let player_texture_sample = player_texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest);

        // Build the shader programs
        let cube_program = glium::Program::from_source(&display, CUBE_VERTEX_SHADER, CUBE_FRAGMENT_SHADER, None).unwrap();
        let rect_program = glium::Program::from_source(&display, RECT_VERTEX_SHADER, RECT_FRAGMENT_SHADER, None).unwrap();
        let entity_program = glium::Program::from_source(&display, ENTITY_VERTEX_SHADER, ENTITY_FRAGMENT_SHADER, None).unwrap();

        // Start rendering by creating a new frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        // Last details before running
        self.update_items();

        // Initially, ask for server updates
        self.proxy.lock().unwrap().send_position_update(self.cam.position().clone());
        self.handle_server_updates();

        // Initialize cube_to_draw, this SHOULD NOT go into handle_server_update as it is call at every loop !
        self.world.set_cubes_to_draw();

        // Uniform for rect computed before the loop
        let rect_uniforms = uniform! {
            font_atlas: &font_atlas,
            font_offsets: GLChar::get_offset(),
            textures: cubes_texture_sampler
        };

        // Event loop
        let mut t = Instant::now();
        let initial_waiting_delay = Duration::from_millis(self.proxy.lock().unwrap().loading_delay());
        let mut is_initializing = true;
        event_loop.run(move |event, window_target| {

            // This block is here to leave some initial time for the world to load updates.
            if is_initializing && t.elapsed() < initial_waiting_delay {
                return;
            } else if is_initializing {
                is_initializing = false;
                t = Instant::now();
            }

            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::Resized(_) => {
                        self.hud_renderer.set_dimension(display.get_framebuffer_dimensions());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        let mut target = display.draw();
                        target.clear_color_and_depth(Color::Sky1.to_tuple(), 1.0);

                        // Step the camera with the elapsed time
                        let dt = t.elapsed();
                        if self.cam.is_selecting_cube() && self.is_left_clicking {
                            self.click_time += dt.as_secs_f32();
                            if self.click_time >= CLICK_TIME_TO_BREAK {
                                // Break the cube
                                self.apply_action(Destroy { at: self.cam.selected_cube().unwrap().to_cube_coordinates() });
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
                                .set_debug(DebugData::new(self.fps_manager.fps(), self.cam.position().clone(), self.world.number_cubes_rendered()));
                        }

                        // I) Draw the cubes

                        // Configure the GPU to do Depth testing (with a depth buffer)
                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                                write: true,
                                ..Default::default()
                            },
                            blend: glium::draw_parameters::Blend::alpha_blending(),
                            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
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

                        // We use OpenGL's instancing feature which allows us to render huge amounts ot cubes at once.
                        // OpenGL instancing = instead of setting 1000 times different uniforms, you give once 1000 attributes
                        let position_buffer = self.world.get_cubes_buffer(&display, self.cam.selected_cube());
                        target.draw(
                            (&cube_vertex_buffer, position_buffer.per_instance().unwrap()),
                            &indices,
                            &cube_program,
                            &uniforms,
                            &params).unwrap();

                        // II) Draw the entity

                        // Define our uniforms (same uniforms for all cubes)...
                        let entity_uniforms = uniform! {
                            view: self.cam.view_matrix(),
                            perspective: self.cam.perspective_matrix(target.get_dimensions()),
                            textures: player_texture_sample
                        };

                        // Prepare the entity buffer to send to the gpu
                        // TODO why is this dynamic and not immutable ?
                        let entity_buffer = glium::VertexBuffer::dynamic(&display, &mut self.entity_manager.get_opengl_entities()).unwrap();
                        target.draw(
                            (&cube_vertex_buffer, entity_buffer.per_instance().unwrap()),
                            &indices,
                            &entity_program,
                            &entity_uniforms,
                            &params).unwrap();

                        // III) Drawn the tiles

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
                        KeyCode::Digit1 => {
                            self.items.set_current_item(0);
                            self.update_items();
                        },
                        KeyCode::Digit2 => {
                            self.items.set_current_item(1);
                            self.update_items();
                        },
                        KeyCode::Digit3 => {
                            self.items.set_current_item(2);
                            self.update_items();
                        },
                        KeyCode::Digit4 => {
                            self.items.set_current_item(3);
                            self.update_items();
                        },
                        KeyCode::Digit5 => {
                            self.items.set_current_item(4);
                            self.update_items();
                        },
                        KeyCode::Digit6 => {
                            self.items.set_current_item(5);
                            self.update_items();
                        },
                        KeyCode::Digit7 => {
                            self.items.set_current_item(6);
                            self.update_items();
                        },
                        KeyCode::Digit8 => {
                            self.items.set_current_item(7);
                            self.update_items();
                        },
                        KeyCode::Digit9 => {
                            self.items.set_current_item(8);
                            self.update_items();
                        },
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
            Add { at: _, block } => {
                self.items.consume(block);
            }
        }

        // Currently, all actions end up editing the items.
        self.update_items();

        // Handle cubes
        self.world.apply_action(&action);

        // Forward to server
        self.proxy.lock().unwrap().on_new_action(action);
    }

    fn update_items(&mut self) {
        self.hud_renderer.set_player_items(self.items.get_current_items(), self.items.current_item());
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
            if let Some((touched_cube, touched_pos)) = self.cam.selection_internals() {
                if let Some(block) = self.items.get_current_block() {
                    self.apply_action(Action::Add {
                        at: Action::position_to_generate_cube(&touched_cube, self.cam.position().pos(), self.cam.direction()),
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
                ServerUpdate::LoggedIn(client_id) => {
                    println!("Client registered ID: {client_id}")
                    // TODO if needed, here is the ID of the player
                }
                ServerUpdate::SendAction(action) => self.world.apply_action(&action),
                ServerUpdate::RegisterEntity(id, pos) => self.entity_manager.register_new_player(id, pos),
                ServerUpdate::UpdatePosition(id, pos) => self.entity_manager.set_position(id, pos),
            }
        }
    }
}