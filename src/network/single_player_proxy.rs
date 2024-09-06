use crate::actions::Action;
use crate::game_server::GameServer;
use crate::network::proxy::Proxy;
use crate::network::server_update::ServerUpdate;
use crate::primitives::position::Position;


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
}

impl Proxy for SinglePlayerProxy {

    fn login(&mut self) {
        self.client_id = self.server.login("client");
    }

    fn send_position_update(&mut self, position: Position) {
        self.server.on_new_position_update(self.client_id, position);
    }

    fn on_new_action(&mut self, action: Action) {
        self.server.on_new_action(self.client_id, action);
    }

    fn consume_server_updates(&mut self) -> Vec<ServerUpdate> {
        self.server.consume_updates(self.client_id)
    }

    fn loading_delay(&self) -> u64 {
        0
    }
}
