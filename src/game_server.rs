use crate::actions::Action;
use crate::chunk::Chunk;
use crate::server_update::ServerUpdate;
use crate::vector::Vector3;
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
            server_updates_buffer: Vec::new()
        }
    }

    /// Logins a new player into the server
    /// Returns the ID of the registered player
    pub fn login(&mut self, name: &str) -> usize {
        let id = self.n_players;
        println!("[SERVER] New player registered: {name} (ID={})", id);
        self.n_players += 1;
        self.server_updates_buffer.push(Vec::new());
        self.world_dispatcher.register_player(id);
        id
    }

    // Implementation of the 'callbacks': entry points of the server

    /// Called when receiving the position of a new player
    pub fn on_new_position_update(&mut self, player_id: usize, position: Vector3) {
        println!("New pos for player {player_id}");
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
    }

    pub fn on_new_action(&mut self, client: usize, action: Action) {
        self.world.apply_action(&action);
    }

    /// Returns the list of updates that the server sends to the client.
    pub fn consume_updates(&mut self, player_id: usize) -> Vec<ServerUpdate> {
        let updates  = self.server_updates_buffer[player_id].clone();
        self.server_updates_buffer[player_id] = Vec::new();
        updates
    }
}

