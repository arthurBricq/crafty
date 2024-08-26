use crate::server::{Server, ServerUpdate};
use crate::vector::Vector3;

pub struct SinglePlayerProxy {
    server: Server,
    client_id: usize
}

impl SinglePlayerProxy {
    pub fn new(server: Server) -> Self {
        Self {
            server,
            client_id: 0,
        }
    }
    
    pub fn login(&mut self) {
        self.client_id = self.server.login("client");
    }
    
    pub fn send_position_update(&mut self, position: Vector3) -> Vec<ServerUpdate> {
        self.server.on_new_position_update(self.client_id, position);
        self.server.get_updates(self.client_id)
    }
    
}
