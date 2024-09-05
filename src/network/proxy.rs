use crate::actions::Action;
use crate::network::server_update::ServerUpdate;
use crate::primitives::vector::Vector3;

/// Defines the interface that a client use to communicate with the game server
pub trait Proxy {
    fn login(&mut self);
    fn send_position_update(&mut self, position: Vector3);
    fn on_new_action(&mut self, action: Action);
    fn consume_server_updates(&mut self) -> Vec<ServerUpdate>;
    /// Returns the delay to wait for at startup
    fn loading_delay(&self) -> u64;
}
