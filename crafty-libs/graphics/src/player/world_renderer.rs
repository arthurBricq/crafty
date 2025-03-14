extern crate glium;
extern crate winit;

// use std::ops::ControlFlow;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{uniform, Surface};
use winit::event::ElementState::Pressed;
use winit::event::{AxisId, ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Fullscreen, Window};
use model::entity::entity_manager::EntityManager;
use model::game::actions::Action;
use model::game::actions::Action::{Add, Destroy};
use model::game::camera::perspective_matrix;
use model::game::health::Health;
use model::game::input::MotionState;
use model::game::player::{Player, CLICK_TIME_TO_BREAK};
use model::game::player_items::PlayerItems;
use model::primitives::position::Position;
use model::server::server_update::ServerUpdate;
use model::world::block_kind::Block::{COBBELSTONE, OAKLOG, SWORD};
use model::world::world::World;
use network::proxy::Proxy;
use crate::core::color::Color;
use crate::core::cube::{CUBE_FRAGMENT_SHADER, CUBE_VERTEX_SHADER, VERTICES};
use crate::core::entity::{ENTITY_FRAGMENT_SHADER, ENTITY_VERTEX_SHADER};
use crate::core::font::GLChar;
use crate::core::hud_renderer::HUDRenderer;
use crate::core::inventory_event::InventoryEvent;
use crate::core::menu_debug::DebugData;
use crate::core::rectangle::{RECT_FRAGMENT_SHADER, RECT_VERTEX_SHADER, RECT_VERTICES};
use crate::core::texture;
use crate::player::fps::FpsManager;

/// 16ms => 60 FPS roughly
const TARGET_FRAME_DURATION: Duration = Duration::from_millis(16);
/// If the frame is `MIN_SLEEP_TIME` shorter than the target duration or less,
/// does not sleep, because of granularity of time in `std::thread::sleep`
const MIN_SLEEP_TIME: Duration = Duration::from_millis(2);

/// The struct in charge of drawing the world
pub struct WorldRenderer {
    /// Link with the server
    /// The proxy needs to be already logged-in
    proxy: Arc<Mutex<dyn Proxy>>,

    /// Currently displayed world
    world: World,

    /// Position and orientation of the player
    player: Player,

    /// Items of the player
    items: PlayerItems,

    /// Health of the player
    health: Health,

    /// In charge of rendering of the 2D menus on the screen
    hud_renderer: HUDRenderer,

    /// In charge of renderin all the other entites
    entity_manager: EntityManager,

    /// Computes the current FPS
    fps_manager: FpsManager,

    /// Is the window displayed in fullscreen ?
    fullscreen: bool,
}

impl WorldRenderer {
    pub fn new(proxy: Arc<Mutex<dyn Proxy>>, world: World, player: Player) -> Self {
        Self {
            proxy,
            world,
            player,
            health: Health::new(10),
            hud_renderer: HUDRenderer::new(),
            fps_manager: FpsManager::new(),
            items: PlayerItems::empty(),

            fullscreen: false,
            entity_manager: EntityManager::new(),
        }
    }

    pub fn run(&mut self) {
        // We start by creating the EventLoop, this can only be done once per process.
        // This also needs to happen on the main thread to make the program portable.
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("event loop building");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Crafty")
            // In macos, you basically can't resize the window...
            // There is a stretching bug, and I am not sure if it is a `glutin` bug or if it is implementation bug
            // from my side, due to a different OpenGL implementation on MacOS.
            // What I can do for is to create a "big enough" window
            .with_inner_size(3460, 2000)
            .build(&event_loop);

        // Add a few items
        self.items.collect(SWORD);
        for _ in 0..16 {
            self.items.collect(COBBELSTONE);
        }
        for _ in 0..8 {
            self.items.collect(OAKLOG);
        }

        let lock_mouse = window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked));
        
        if lock_mouse.is_err() {
            println!("Could not lock the mouse")
        }

        #[cfg(not(target_os = "macos"))]
        {
            window.set_cursor_visible(false);
        }

        // Construct the buffer of vertices (for single objects, we use OpenGL's instancing to multiply them)
        let cube_vertex_buffer = glium::VertexBuffer::new(&display, &VERTICES).unwrap();
        let rect_vertex_buffer = glium::VertexBuffer::new(&display, &RECT_VERTICES).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // Build the texture library, and change the sampler to use the proper filters
        let textures = texture::build_textures_array(&display);
        let cubes_texture_sampler = textures
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest);

        // Load other textures that are used
        let selected_texture = texture::load_texture(
            std::fs::read("./resources/selected.png")
                .unwrap()
                .as_slice(),
            &display,
        );
        let font_atlas = texture::load_texture(
            std::fs::read("./resources/fonts.png").unwrap().as_slice(),
            &display,
        );

        // Textures for entities
        let humanoid_texture = texture::load_humanoid_textures("./resources/entity/", &display);
        let humanoid_texture_sample = humanoid_texture
            .sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest);

        // Build the shader programs
        let cube_program =
            glium::Program::from_source(&display, CUBE_VERTEX_SHADER, CUBE_FRAGMENT_SHADER, None)
                .unwrap();
        let rect_program =
            glium::Program::from_source(&display, RECT_VERTEX_SHADER, RECT_FRAGMENT_SHADER, None)
                .unwrap();
        let entity_program = glium::Program::from_source(
            &display,
            ENTITY_VERTEX_SHADER,
            ENTITY_FRAGMENT_SHADER,
            None,
        )
        .unwrap();
        // Start rendering by creating a new frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        // Last details before running
        self.update_items_bar();

        // Initially, ask for server updates
        self.proxy
            .lock()
            .unwrap()
            .send_position_update(self.player.position().clone());
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
        let initial_waiting_delay =
            Duration::from_millis(self.proxy.lock().unwrap().loading_delay());
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
                    winit::event::WindowEvent::Resized(tmp) => {
                        println!("Resized to {:?}", tmp);
                        self.hud_renderer.set_dimension(display.get_framebuffer_dimensions());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        // Step the camera with the elapsed time
                        let dt = t.elapsed();
                        t = Instant::now();

                        let mut target = display.draw();
                        target.clear_color_and_depth(Color::Sky1.to_tuple(), 1.0);

                        // Step the camera with the elapsed time
                        // Try to break the selected cube
                        if self.player.is_time_to_break_over(dt.as_secs_f32()) {
                            self.apply_action(Destroy { at: self.player.selected_cube().unwrap().to_cube_coordinates() });
                        }

                        // Step
                        self.fps_manager.step(dt);
                        self.player.step(dt, &self.world);

                        // Server updates
                        self.proxy.lock().unwrap().send_position_update(self.player.position().clone());
                        self.handle_server_updates();

                        // HUD updates
                        if self.hud_renderer.show_debug() {
                            self.hud_renderer
                                .set_debug(DebugData::new(self.fps_manager.fps(), self.player.position().clone(), self.world.number_cubes_rendered()));
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
                            view: self.player.view_matrix(),
                            perspective: perspective_matrix(target.get_dimensions()),
                            textures: cubes_texture_sampler,
                            selected_texture: &selected_texture,
                            selected_intensity: if self.player.left_click() {self.player.left_click_time() / CLICK_TIME_TO_BREAK} else {0.2},
                        };

                        // We use OpenGL's instancing feature which allows us to render huge amounts ot cubes at once.
                        // OpenGL instancing = instead of setting 1000 times different uniforms, you give once 1000 attributes
                        let position_buffer = self.world.get_cubes_buffer(&display, self.player.selected_cube());
                        target.draw(
                            (&cube_vertex_buffer, position_buffer.per_instance().unwrap()),
                            &indices,
                            &cube_program,
                            &uniforms,
                            &params).unwrap();

                        // II) Draw the entity

                        // Define our uniforms (same uniforms for all cubes)...
                        let entity_uniforms = uniform! {
                            view: self.player.view_matrix(),
                            perspective: perspective_matrix(target.get_dimensions()),
                            entity_textures: humanoid_texture_sample,
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
                    winit::event::WindowEvent::MouseInput { device_id: _, state, button } => {
                        if !self.hud_renderer.is_inventory_open() {
                            self.handle_button_event(button, state)
                        }

                        // left click
                        if button == MouseButton::Left {
                            self.hud_renderer.maybe_forward_inventory_event(InventoryEvent::Button(state))
                        }
                    }
                    winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => self.handle_key_event(event, &window),
                    // Inventory requires us to deal with cursor events, not mouse events
                    // TODO capture nicely the events for the inventory
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        let x: f32 = -1. + 2. * position.x as f32 / window.inner_size().width as f32;
                        let y: f32 = 1. - 2. * position.y as f32 / window.inner_size().height as f32;
                        self.hud_renderer.maybe_forward_inventory_event(InventoryEvent::CursorMoved(x, y));
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    let opt_time_to_sleep = (t + TARGET_FRAME_DURATION - MIN_SLEEP_TIME).checked_duration_since(Instant::now());

                    if let Some(time_to_sleep) = opt_time_to_sleep {
                        std::thread::sleep(time_to_sleep + MIN_SLEEP_TIME);
                    }
                    window.request_redraw()
                }
                winit::event::Event::DeviceEvent { event, .. } => match event {
                    winit::event::DeviceEvent::Motion { axis, value } => self.handle_motion_event(axis, value),
                    _ => {}
                }
                _ => (),
            };
        }).unwrap();
    }

    fn handle_key_event(&mut self, event: KeyEvent, window: &Window) {
        self.handle_general_key_event(&event);

        if self.hud_renderer.is_inventory_open() {
            self.handle_inventory_key_event(event, window)
        } else {
            self.handle_game_key_event(event, window)
        }
    }

    /// Deal with the events that needs to be taken care of both in the
    /// inventory and in the game
    fn handle_general_key_event(&mut self, event: &KeyEvent) {
        if event.state == Pressed {
            match event.physical_key {
                PhysicalKey::Code(key) => match key {
                    KeyCode::Escape => std::process::exit(1),
                    _ => {}
                },
                PhysicalKey::Unidentified(_) => {}
            }
        }
    }

    fn handle_inventory_key_event(&mut self, event: KeyEvent, window: &Window) {
        if event.state == Pressed {
            match event.physical_key {
                PhysicalKey::Code(key) => match key {
                    KeyCode::KeyE => {
                        if let Some(items) = self.hud_renderer.close_inventory() {
                            self.items = items;
                            window.set_cursor_visible(false);
                            self.update_items_bar();
                        }
                    }
                    _ => {}
                },
                PhysicalKey::Unidentified(_) => {}
            }
        }
    }

    fn handle_game_key_event(&mut self, event: KeyEvent, window: &Window) {
        // Handle keys related to motion (toggle is important here)
        if event.repeat {
            return;
        }
        let pressed = event.state.is_pressed();
        match event.physical_key {
            PhysicalKey::Code(key) => match key {
                KeyCode::KeyW => self.player.toggle_state(MotionState::Up, pressed),
                KeyCode::KeyS => self.player.toggle_state(MotionState::Down, pressed),
                KeyCode::KeyD => self.player.toggle_state(MotionState::Right, pressed),
                KeyCode::KeyA => self.player.toggle_state(MotionState::Left, pressed),
                KeyCode::KeyK => self.player.up(),
                KeyCode::KeyJ => self.player.down(),
                KeyCode::Space => self.player.toggle_state(MotionState::Jump, pressed),
                _ => {}
            },
            _ => {}
        }

        // Second match is for other stuff that only needs to be detected when pressed
        if event.state == Pressed {
            match event.physical_key {
                PhysicalKey::Code(key) => {
                    match key {
                        // Inventory
                        KeyCode::KeyE => {
                            self.hud_renderer.open_inventory(self.items.clone());
                            window.set_cursor_visible(true);
                        }

                        // Item bar shortcuts
                        KeyCode::Digit1 => {
                            self.items.set_current_item(0);
                            self.update_items_bar();
                        }
                        KeyCode::Digit2 => {
                            self.items.set_current_item(1);
                            self.update_items_bar();
                        }
                        KeyCode::Digit3 => {
                            self.items.set_current_item(2);
                            self.update_items_bar();
                        }
                        KeyCode::Digit4 => {
                            self.items.set_current_item(3);
                            self.update_items_bar();
                        }
                        KeyCode::Digit5 => {
                            self.items.set_current_item(4);
                            self.update_items_bar();
                        }
                        KeyCode::Digit6 => {
                            self.items.set_current_item(5);
                            self.update_items_bar();
                        }
                        KeyCode::Digit7 => {
                            self.items.set_current_item(6);
                            self.update_items_bar();
                        }
                        KeyCode::Digit8 => {
                            self.items.set_current_item(7);
                            self.update_items_bar();
                        }
                        KeyCode::Digit9 => {
                            self.items.set_current_item(8);
                            self.update_items_bar();
                        }
                        KeyCode::KeyP => {
                            println!("=================");
                            println!("Debug Information");
                            println!("=================");
                            self.player.debug();
                        }
                        KeyCode::KeyX => {
                            println!("Ask to spawn a monster");
                            let mut monster_pos =
                                Position::new(self.player.position().pos().clone(), 0., 0.);
                            monster_pos.small_raise();
                            self.proxy.lock().unwrap().request_to_spawn(monster_pos);
                        }
                        KeyCode::F3 => self.hud_renderer.toggle_debug_menu(),
                        KeyCode::F10 => self.toggle_fullscreen(&window),
                        KeyCode::F11 => self.world.save_to_file("map.json"),
                        KeyCode::F12 => self.hud_renderer.toggle_help_menu(),
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
                    self.items.collect(block.block_dropped());
                }
            }
            Add { at, block } => {
                if self.player.is_in(at) {
                    return; // cannot place a block on oneself
                }

                // cannot place some blocks (i.e. swords)
                if let Some(block) = self.items.get_current_block() {
                    if !block.can_be_placed() {
                        return;
                    }
                }

                self.items.consume(block);
            }
        }

        // Currently, all actions end up editing the items.
        self.update_items_bar();

        // Handle cubes
        self.world.apply_action(&action);

        // Forward to server
        self.proxy.lock().unwrap().on_new_action(action);
    }

    fn update_items_bar(&mut self) {
        self.hud_renderer
            .set_player_items(self.items.get_bar_items(), self.items.current_item());
    }

    fn handle_button_event(&mut self, button: MouseButton, state: ElementState) {
        match button {
            MouseButton::Left => {
                if self.player.is_selecting_cube() {
                    self.player
                        .toggle_state(MotionState::LeftClick, state.is_pressed());
                } else if state.is_pressed() {
                    if let Some(mut attack) = self
                        .entity_manager
                        .attack(self.player.position().pos(), self.player.direction())
                    {
                        // Forward the attack to the server
                        attack.set_strength(self.items.attack_strength());
                        self.proxy.lock().unwrap().on_new_attack(attack);
                    }
                }
            }
            MouseButton::Right => {
                if state == Pressed {
                    // Right click = add a new cube
                    // We know where is the player and we know
                    if let Some(touched_cube) = self.player.selected_cube() {
                        if let Some(block) = self.items.get_current_block() {
                            // TODO Sometimes, I can't understand why, we end up not finding a place to add the new cube.
                            //      I know this is not easy to debug...
                            if let Ok(at) = touched_cube.position_to_add_new_cube(
                                self.player.position().pos(),
                                self.player.direction(),
                            ) {
                                self.apply_action(Action::Add { at, block })
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn handle_motion_event(&mut self, axis: AxisId, value: f64) {
        // TODO make something cleaner
        if !self.hud_renderer.is_inventory_open() {
            if axis == 0 {
                self.player.mousemove(value as f32, 0.0, 0.005);
            } else {
                self.player.mousemove(0.0, -value as f32, 0.005);
            }
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
                ServerUpdate::LoggedIn(client_id, position) => {
                    println!("Client registered ID: {client_id} with position: {position:?}");
                    self.player.set_position(position)
                }
                ServerUpdate::SendAction(action) => self.world.apply_action(&action),
                ServerUpdate::RegisterEntity(id, entity_kind, pos) => self
                    .entity_manager
                    .register_new_entity(id, entity_kind, pos),
                ServerUpdate::UpdatePosition(id, pos) => self.entity_manager.set_position(id, pos),
                ServerUpdate::Attack(attack) => {
                    self.health.damage(attack.strength());
                    self.hud_renderer.set_health(&self.health);
                }
                ServerUpdate::RemoveEntity(id) => self.entity_manager.remove_entity(id as u8),
            }
        }
    }
}
