use model::game::actions::Action;
use model::game::attack::EntityAttack;
use model::server::server_update::ServerUpdate;
use primitives::position::Position;

/// Defines the interface for client-to-server communication
pub trait ClientToServer: Send + Sync {
    async fn login(&mut self, name: String);
    async fn send_position_update(&mut self, position: Position);
    async fn on_new_action(&mut self, action: Action);
    async fn on_new_attack(&mut self, attack: EntityAttack);
    async fn request_to_spawn(&mut self, position: Position);
}

/// Defines the interface for server-to-client communication
pub trait ServerToClient: Send + Sync {
    /// Returns the next server update, if available
    /// Returns None when there are no more updates currently available
    async fn next_updates(&mut self) -> Option<Vec<ServerUpdate>>;
}

/// Convenience trait that combines both directions of communication
/// This allows types to implement both traits if needed
pub trait Proxy: ClientToServer + ServerToClient {}
