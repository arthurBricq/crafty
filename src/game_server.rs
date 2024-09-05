use crate::actions::Action;
use crate::chunk::CHUNK_FLOOR;
use crate::network::server_update::ServerUpdate;
use crate::network::server_update::ServerUpdate::{LoggedIn, RegisterEntity, SendAction, UpdatePosition};
use crate::primitives::vector::Vector3;
use crate::world::World;
use crate::world_dispatcher::WorldDispatcher;

/// The GameServer is the model of the server
pub struct GameServer {
    /// The full world
    world: World,

    /// number of players
    n_players: usize,

    /// In charge of telling which chunks must be loaded by which player
    world_dispatcher: WorldDispatcher,

    /// Buffer of updates to be sent to each player
    server_updates_buffer: Vec<Vec<ServerUpdate>>,
}

impl GameServer {
    pub fn new(world: World) -> Self {
        Self {
            world,
            n_players: 0,
            world_dispatcher: WorldDispatcher::new(),
            server_updates_buffer: Vec::new(),
        }
    }

    /// Logins a new player into the server
    /// Returns the ID of the registered player
    pub fn login(&mut self, name: &str) -> usize {
        // Create the new ID
        let id = self.n_players;
        self.n_players += 1;
        println!("[SERVER] New player registered: {name} (ID={})", id);

        // Create a new buffer of updates, and initialize it directly with a LoggedIn message and the position of the other players
        let mut initial_updates = vec![LoggedIn(id as u8)];
        for i in 0..self.n_players - 1 {
            // TODO find the actual position of each player...
            initial_updates.push(RegisterEntity(i as u8, Vector3::newi(0, CHUNK_FLOOR as i32 + 2, 0)))
        }

        self.server_updates_buffer.push(initial_updates);


        // Register the player in the dispatcher
        self.world_dispatcher.register_player(id);

        // Register the player to other players of the game.
        for i in 0..self.n_players - 1 {
            self.server_updates_buffer[i].push(RegisterEntity(id as u8, Vector3::newi(0, CHUNK_FLOOR as i32 + 2, 0)))
        }

        id
    }

    // Implementation of the 'callbacks': entry points of the server

    /// Called when receiving the position of a new player
    pub fn on_new_position_update(&mut self, player_id: usize, position: Vector3) {
        // Update the world dispatcher. to compute if the player needs to be sent new chunks
        if let Some((chunks_to_send, chunks_to_delete)) = self.world_dispatcher.update_position(player_id, (position.x(), position.z())) {
            let buffer = &mut self.server_updates_buffer[player_id];
            for corner in chunks_to_send {
                // Find the correct chunk
                if let Some(to_send) = self.world.get_chunk(corner) {
                    buffer.push(ServerUpdate::LoadChunk(to_send))
                } else {
                    // TODO generate chunk !!!
                }
            }
        }

        // Update other players
        for i in 0..self.n_players {
            if i != player_id {
                self.server_updates_buffer[i].push(UpdatePosition(player_id as u8, position))
            }
        }

    }

    pub fn on_new_action(&mut self, player_id: usize, action: Action) {
        // Edit the world of the server
        self.world.apply_action(&action);
        // Forward the action to all OTHER clients
        for i in 0..self.n_players {
            if i != player_id {
                self.server_updates_buffer[i].push(SendAction(action.clone()))
            }
        }
    }

    /// Returns the list of updates that the server sends to the client.
    pub fn consume_updates(&mut self, player_id: usize) -> Vec<ServerUpdate> {
        // if player_id < self.n_players {return vec![]}
        let updates = self.server_updates_buffer[player_id].clone();
        self.server_updates_buffer[player_id] = Vec::new();
        updates
    }
}

