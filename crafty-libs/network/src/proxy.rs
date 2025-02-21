use model::game::actions::Action;
use model::game::attack::EntityAttack;
use model::primitives::position::Position;
use model::server::server_update::ServerUpdate;

/// Defines the interface that a client use to communicate with the game server
pub trait Proxy {
    fn login(&mut self, name: String);
    fn send_position_update(&mut self, position: Position);
    fn on_new_action(&mut self, action: Action);
    fn on_new_attack(&mut self, attack: EntityAttack);
    fn request_to_spawn(&mut self, position: Position);
    fn consume_server_updates(&mut self) -> Vec<ServerUpdate>;
    /// Returns the delay to wait for at startup
    fn loading_delay(&self) -> u64;
}
