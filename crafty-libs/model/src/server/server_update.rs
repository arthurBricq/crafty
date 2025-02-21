use crate::entity::entity::EntityKind;
use crate::game::actions::Action;
use crate::game::attack::EntityAttack;
use crate::primitives::position::Position;
use crate::server::server_update::ServerUpdate::LoadChunk;
use crate::world::chunk::Chunk;

pub const RESPONSE_OK: u8 = 100;
pub const RESPONSE_ERROR: u8 = 101;

/// List of messages that are sent to the client from the server
#[derive(Clone, Debug)]
pub enum ServerUpdate {
    /// Ask the client to load a new chunk
    LoadChunk(Chunk),
    /// The server forwards to the client his client id
    LoggedIn(u8, Position),
    /// The server forwards to the client an action to be executed
    SendAction(Action),
    /// Tell the client that a new player is part of the game
    RegisterEntity(u8, EntityKind, Position),
    /// Update the position of an existing entity
    UpdatePosition(u8, Position),
    /// Attack to suffer... :(
    Attack(EntityAttack),
    /// Remove an entity
    RemoveEntity(u32),
}

impl ServerUpdate {
    /// Returns true if this update is big enough to require a special treatment when sent over the
    /// network.
    pub fn is_heavy(&self) -> bool {
        matches!(self, LoadChunk(_))
    }
}

