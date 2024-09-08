use crate::actions::Action;
use crate::network::server_update::ServerUpdate;
use crate::network::server_update::ServerUpdate::{LoggedIn, RegisterEntity, SendAction, UpdatePosition};
use crate::primitives::position::Position;
use crate::server::monster_manager::MonsterManager;
use crate::server::server_state::ServerState;
use crate::server::world_dispatcher::WorldDispatcher;
use crate::world::World;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Main function of the thread in charge of entities
pub fn handle_entity_thread(server: Arc<Mutex<GameServer>>) {
    let sleep_time = Duration::from_millis(20);

    loop {
        server.lock().unwrap().entity_server.step(sleep_time);
        std::thread::sleep(sleep_time)
    }
}

/// The GameServer is the model of the server
pub struct GameServer {
    /// The full world
    world: Arc<Mutex<World>>,

    /// In charge of telling which chunks must be loaded by which player
    world_dispatcher: WorldDispatcher,

    /// Buffer of updates to be sent to each player
    server_updates_buffer: HashMap<usize, Vec<ServerUpdate>>,

    /// In charge of handling the entities
    entity_server: MonsterManager,
    
    /// Internal state of the server (expect the entities)
    state: ServerState
}

impl GameServer {
    pub fn new(world: World) -> Self {
        let ref_to_world = Arc::new(Mutex::new(world));
        Self {
            world: Arc::clone(&ref_to_world),
            world_dispatcher: WorldDispatcher::new(),
            server_updates_buffer: HashMap::new(),
            entity_server: MonsterManager::new(ref_to_world),
            state: ServerState::new()
        }
    }

    /// Logins a new player into the server
    /// Returns the ID of the registered player
    pub fn login(&mut self, name: String) -> usize {
        // Create the new ID
        let player = self.state.login(name.clone());
        println!("[SERVER] New player registered: {name} (ID={}, pos={:?})", player.id, player.pos);
        println!("Connected players: {}", self.state.n_players_connected());
        
        // Create a new buffer of updates for this client, 
        let mut initial_updates = vec![LoggedIn(player.id as u8, player.pos.clone())];

        // Initialize it directly with a LoggedIn message and the position of the other players
        for (i, connected) in self.state.connected_players().enumerate() {
            if connected.id != player.id {
                initial_updates.push(RegisterEntity(i as u8, player.pos.clone()))
            }
        }
        
        self.server_updates_buffer.insert(player.id, initial_updates);

        // Register the player in the dispatcher
        self.world_dispatcher.register_player(player.id);

        // Register the new player to other players of the game.
        for i in 0..self.state.n_players_connected() - 1 {
            self.server_updates_buffer.get_mut(&i).unwrap().push(RegisterEntity(player.id as u8, player.pos.clone()));
        }

        player.id
    }

    pub fn logout(&mut self, id: usize) {
        // The world dispatcher must be informed that this client loose all of its chunks
        self.state.logout(id);
        self.world_dispatcher.logout(id);
    }

    // Implementation of the 'callbacks': entry points of the server

    /// Called when receiving the position of a new player
    pub fn on_new_position_update(&mut self, player_id: usize, position: Position) {
        // Update the world dispatcher. to compute if the player needs to be sent new chunks
        if let Some((chunks_to_send, _chunks_to_delete)) = self.world_dispatcher.update_position(player_id, (position.x(), position.z())) {
            for corner in chunks_to_send {
                // Find the correct chunk
                if let Some(to_send) = self.world.lock().unwrap().get_chunk(corner) {
                    self.server_updates_buffer.get_mut(&player_id).unwrap().push(ServerUpdate::LoadChunk(to_send))
                } else {
                    // TODO generate chunk !!!
                }
            }
        }

        // Update other players
        for i in 0..self.state.n_players_connected() {
            if i != player_id {
                self.server_updates_buffer.get_mut(&i).unwrap().push(UpdatePosition(player_id as u8, position.clone()))
            }
        }

        // Update internal state
        self.state.set_player_pos(player_id, position.clone());
    }

    pub fn on_new_action(&mut self, player_id: usize, action: Action) {
        // Edit the world of the server
        self.world.lock().unwrap().apply_action(&action);
        // Forward the action to all OTHER clients
        for i in 0..self.state.n_players_connected() {
            if i != player_id {
                self.server_updates_buffer.get_mut(&i).unwrap().push(SendAction(action.clone()))
            }
        }
    }

    /// Returns the list of updates that the server sends to the client.
    pub fn consume_updates(&mut self, player_id: usize) -> Vec<ServerUpdate> {
        let mut updates_for_player = self.server_updates_buffer.insert(player_id, Vec::new()).unwrap();
        // Add to these updates the ones that the entity manager also provides
        // TODO use the position of the player in the server state
        updates_for_player.extend_from_slice(&self.entity_server.get_server_updates(Position::empty()));
        updates_for_player
    }
}

