use crate::proxy::{ClientToServer, Proxy, ServerToClient};
use model::game::actions::Action;
use model::game::attack::EntityAttack;
use model::server::game_server::GameServer;
use model::server::server_update::ServerUpdate;
use primitives::position::Position;
use std::sync::{Arc, Mutex};

pub struct SinglePlayerProxy {
    server: Arc<Mutex<GameServer>>,
    client_id: usize,
}

impl SinglePlayerProxy {
    pub fn new(server: Arc<Mutex<GameServer>>) -> Self {
        Self {
            server,
            client_id: 0,
        }
    }
}

impl ClientToServer for SinglePlayerProxy {
    async fn login(&mut self, name: String) {
        self.client_id = self.server.lock().unwrap().login(name);
    }

    async fn send_position_update(&mut self, position: Position) {
        self.server
            .lock()
            .unwrap()
            .on_new_position_update(self.client_id, position);
    }

    async fn on_new_action(&mut self, action: Action) {
        self.server
            .lock()
            .unwrap()
            .on_new_action(self.client_id, action);
    }

    async fn on_new_attack(&mut self, attack: EntityAttack) {
        self.server.lock().unwrap().on_new_attack(attack);
    }

    async fn request_to_spawn(&mut self, position: Position) {
        self.server.lock().unwrap().spawn_monster(position);
    }
}

impl ServerToClient for SinglePlayerProxy {
    async fn next_updates(&mut self) -> Option<Vec<ServerUpdate>> {
        Some(self.server.lock().unwrap().consume_updates(self.client_id))
    }
}

impl Proxy for SinglePlayerProxy {}
