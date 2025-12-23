use crate::core::hud_renderer::HUDRenderer;
use crate::core::inventory_event::InventoryEvent;
use crate::core::menu_debug::DebugData;
use crate::player::fps::FpsManager;
use crate::renderer::{
    KeyCode, KeyEvent, MouseButton, MouseEvent, Renderer, RendererBackend, ToDraw, WindowAction,
};
use model::entity::entity_manager::EntityManager;
use model::game::actions::Action;
use model::game::actions::Action::{Add, Destroy};
use model::game::health::Health;
use model::game::input::MotionState;
use model::game::player::{Player, CLICK_TIME_TO_BREAK};
use model::game::player_items::PlayerItems;
use model::server::server_update::ServerUpdate;
use model::world::block_kind::Block::{COBBELSTONE, OAKLOG, SWORD};
use model::world::chunk::CHUNK_FLOOR;
use model::world::world::World;
use network::proxy::Proxy;
use primitives::position::Position;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

impl RendererBackend for WorldRenderer {
    fn update(&mut self, dt: Duration) -> ToDraw {
        // Step the camera with the elapsed time
        // Try to break the selected cube
        if self.player.is_time_to_break_over(dt.as_secs_f32()) {
            self.apply_action(Destroy {
                at: self.player.selected_cube().unwrap().to_cube_coordinates(),
            });
        }

        // Step
        self.fps_manager.step(dt);
        self.player.step(dt, &self.world);

        // Server updates
        self.proxy
            .lock()
            .unwrap()
            .send_position_update(self.player.position().clone());
        self.handle_server_updates();

        // HUD updates
        if self.hud_renderer.show_debug() {
            self.hud_renderer.set_debug(DebugData::new(
                self.fps_manager.fps(),
                self.player.position().clone(),
                self.world.number_cubes_rendered(),
            ));
        };

        // Compute the buffers to be rendered
        let cubes_buffer = self.world.get_cubes_buffer(self.player.selected_cube());
        let entity_buffer = self.entity_manager.get_opengl_entities();
        let hud_buffer = self.hud_renderer.rects();

        ToDraw {
            player_view_matrix: self.player.view_matrix(),
            selected_intensity: if self.player.left_click() {
                self.player.left_click_time() / CLICK_TIME_TO_BREAK
            } else {
                0.2
            },
            cubes_buffer,
            entity_buffer,
            hud_buffer,
        }
    }

    fn set_dimension(&mut self, dimension: (u32, u32)) {
        self.hud_renderer.set_dimension(dimension);
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) {
        if !self.hud_renderer.is_inventory_open() {
            match event.button {
                MouseButton::Left => {
                    if self.player.is_selecting_cube() {
                        self.player
                            .toggle_state(MotionState::LeftClick, event.state.is_pressed());
                    } else if event.state.is_pressed() {
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
                    if event.state.is_pressed() {
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
                                    self.apply_action(Add { at, block })
                                }
                            }
                        }
                    }
                }
                MouseButton::Other => {}
            }
        }

        // left click
        if event.button == MouseButton::Left {
            self.hud_renderer
                .maybe_forward_inventory_event(InventoryEvent::Button(event.state))
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Vec<WindowAction> {
        // Exit the program
        if key_event.state.is_pressed() {
            if let KeyCode::Escape = key_event.key {
                std::process::exit(0);
            }
        }

        let mut actions = Vec::new();
        if self.hud_renderer.is_inventory_open() {
            self.handle_inventory_key_event(key_event, &mut actions);
        } else {
            self.handle_game_key_event(key_event, &mut actions)
        }

        actions
    }

    fn cursor_moved(&mut self, x: f32, y: f32) {
        self.hud_renderer
            .maybe_forward_inventory_event(InventoryEvent::CursorMoved(x, y));
    }

    fn handle_motion_event(&mut self, axis_id: u32, value: f64) {
        if !self.hud_renderer.is_inventory_open() {
            if axis_id == 0 {
                self.player.mousemove(value as f32, 0.0, 0.005);
            } else {
                self.player.mousemove(0.0, -value as f32, 0.005);
            }
        }
    }
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

    pub fn run<R: Renderer>(&mut self) {
        // Add a few items
        self.items.collect(SWORD);
        for _ in 0..16 {
            self.items.collect(COBBELSTONE);
        }
        for _ in 0..8 {
            self.items.collect(OAKLOG);
        }

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

        // Run the game loop
        let renderer = R::default();
        renderer.run(self)
    }

    fn handle_inventory_key_event(&mut self, event: KeyEvent, actions: &mut Vec<WindowAction>) {
        if event.state.is_pressed() {
            match event.key {
                KeyCode::KeyE => {
                    if let Some(items) = self.hud_renderer.close_inventory() {
                        self.items = items;
                        actions.push(WindowAction::SetCursor(false));
                        self.update_items_bar();
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_game_key_event(&mut self, event: KeyEvent, actions: &mut Vec<WindowAction>) {
        // if event.repeat {
        //     return;
        // }

        // Handle keys related to motion (toggle is important here)
        let pressed = event.state.is_pressed();
        match event.key {
            KeyCode::KeyW => self.player.toggle_state(MotionState::Up, pressed),
            KeyCode::KeyS => self.player.toggle_state(MotionState::Down, pressed),
            KeyCode::KeyD => self.player.toggle_state(MotionState::Right, pressed),
            KeyCode::KeyA => self.player.toggle_state(MotionState::Left, pressed),
            KeyCode::KeyK => self.player.up(),
            KeyCode::KeyJ => self.player.down(),
            KeyCode::Space => self.player.toggle_state(MotionState::Jump, pressed),
            _ => {}
        }

        // Second match is for other stuff that only needs to be detected when pressed
        if event.state.is_pressed() {
            match event.key {
                // Inventory
                KeyCode::KeyE => {
                    self.hud_renderer.open_inventory(self.items.clone());
                    actions.push(WindowAction::SetCursor(true))
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
                    tracing::debug!("=================");
                    tracing::debug!("Debug Information");
                    tracing::debug!("=================");
                    self.player.debug();
                }
                KeyCode::KeyX => {
                    tracing::info!("Ask to spawn a monster");
                    let mut monster_pos =
                        Position::new(self.player.position().pos().clone(), 0., 0.);
                    monster_pos.raise(CHUNK_FLOOR as f32);
                    self.proxy.lock().unwrap().request_to_spawn(monster_pos);
                }
                KeyCode::F3 => self.hud_renderer.toggle_debug_menu(),
                KeyCode::F10 => self.toggle_fullscreen(actions),
                KeyCode::F11 => self.world.save_to_file("map.json"),
                KeyCode::F12 => self.hud_renderer.toggle_help_menu(),
                _ => {}
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

    fn toggle_fullscreen(&mut self, actions: &mut Vec<WindowAction>) {
        if self.fullscreen {
            actions.push(WindowAction::SetFullscreen(false));
            self.fullscreen = false;
        } else {
            actions.push(WindowAction::SetFullscreen(true));
            self.fullscreen = true;
        }
    }

    fn handle_server_updates(&mut self) {
        let updates = self.proxy.lock().unwrap().consume_server_updates();
        for update in updates {
            match update {
                ServerUpdate::LoadChunk(chunk) => self.world.add_chunk(chunk),
                ServerUpdate::LoggedIn(client_id, position) => {
                    tracing::info!("Client registered ID: {client_id} with position: {position:?}");
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
