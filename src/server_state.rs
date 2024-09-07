use std::collections::HashMap;
use crate::primitives::position::Position;

struct PlayerState {
    pos: Position,
    id: usize,
}

/// Persistent state of the server
pub struct ServerState {
    players: HashMap<String, PlayerState>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new()
        }
    }

    pub fn login(&mut self, name: String) -> &PlayerState {
        if !self.players.contains_key(&name) {
            self.players.insert(name.clone(), PlayerState { id: 0, pos: Position::empty() });
        }
        self.players.get(&name).unwrap()
    }
}
