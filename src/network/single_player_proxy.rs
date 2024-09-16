use std::sync::{Arc, Mutex};
use crate::actions::Action;
use crate::attack::EntityAttack;
use crate::server::game_server::GameServer;
use crate::network::proxy::Proxy;
use crate::network::server_update::ServerUpdate;
use crate::primitives::position::Position;


pub struct SinglePlayerProxy {
    server: Arc<Mutex<GameServer>>,
    client_id: usize
}

impl SinglePlayerProxy {
    pub fn new(server: Arc<Mutex<GameServer>>) -> Self {
        Self {
            server,
            client_id: 0,
        }
    }
}

impl Proxy for SinglePlayerProxy {

    fn login(&mut self, name: String) {
        self.client_id = self.server.lock().unwrap().login(name);
    }

    fn send_position_update(&mut self, position: Position) {
        self.server.lock().unwrap().on_new_position_update(self.client_id, position);
    }

    fn on_new_action(&mut self, action: Action) {
        self.server.lock().unwrap().on_new_action(self.client_id, action);
    }
    
    fn on_new_attack(&mut self, attack: EntityAttack) {
        self.server.lock().unwrap().on_new_attack(attack);
    }

    fn consume_server_updates(&mut self) -> Vec<ServerUpdate> {
        self.server.lock().unwrap().consume_updates(self.client_id)
    }

    fn loading_delay(&self) -> u64 {
        0
    }
}
