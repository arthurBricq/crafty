use crate::actions::Action;
use crate::game_server::{GameServer, ServerUpdate};
use crate::vector::Vector3;

pub struct SinglePlayerProxy {
    server: GameServer,
    client_id: usize
}

impl SinglePlayerProxy {
    pub fn new(server: GameServer) -> Self {
        Self {
            server,
            client_id: 0,
        }
    }
    
    pub fn login(&mut self) {
        self.client_id = self.server.login("client");
    }
    
    pub fn send_position_update(&mut self, position: Vector3) {
        self.server.on_new_position_update(self.client_id, position);
    }

    pub fn on_new_action(&mut self, action: Action) {
        self.server.on_new_action(self.client_id, action);
    }
    
    pub fn consume_server_updates(&mut self) -> Vec<ServerUpdate> {
        self.server.consume_updates(self.client_id)
    }
    
}
