use crate::actions::Action;
use crate::network::server_update::ServerUpdate;
use crate::primitives::position::Position;

/// Defines the interface that a client use to communicate with the game server
pub trait Proxy {
    fn login(&mut self, name: String);
    fn send_position_update(&mut self, position: Position);
    fn on_new_action(&mut self, action: Action);
    fn consume_server_updates(&mut self) -> Vec<ServerUpdate>;
    /// Returns the delay to wait for at startup
    fn loading_delay(&self) -> u64;
}
