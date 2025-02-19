use crate::primitives::position::Position;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct PlayerState {
    pub pos: Position,
    pub id: usize,
}

/// Persistent state of the server
pub struct ServerState {
    players: HashMap<String, PlayerState>,
    connected: HashSet<String>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            connected: HashSet::new(),
        }
    }

    pub fn login(&mut self, name: String) -> PlayerState {
        self.connected.insert(name.clone());
        if !self.players.contains_key(&name) {
            self.players.insert(
                name.clone(),
                PlayerState {
                    id: self.players.len(),
                    pos: Position::spawn_position(),
                },
            );
        }
        self.players.get_mut(&name).map(|player| player.pos.raise());
        self.players.get(&name).unwrap().clone()
    }

    pub fn logout(&mut self, id: usize) {
        let name = self.players.iter().find(|(_, v)| v.id == id).unwrap().0;
        self.connected.remove(name);
    }

    pub fn connected_players(&self) -> impl Iterator<Item = &PlayerState> {
        self.players
            .iter()
            .filter(|(k, v)| self.connected.contains(*k))
            .map(|(k, v)| v)
    }

    pub fn n_players_connected(&self) -> usize {
        self.connected.len()
    }

    pub fn set_player_pos(&mut self, id: usize, pos: Position) {
        if let Some(player_state) = self
            .players
            .iter_mut()
            .find(|(k, v)| v.id == id)
            .map(|(k, v)| v)
        {
            player_state.pos = pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::server::server_state::{PlayerState, ServerState};

    #[test]
    fn test_client_id_assignation() {
        let mut state = ServerState::new();

        let p1 = state.login("arthur".to_string());
        assert_eq!(1, state.n_players_connected());
        assert_eq!(0, p1.id);

        let p2 = state.login("johan".to_string());
        assert_eq!(2, state.n_players_connected());
        assert_eq!(1, p2.id);

        state.logout(0);
        assert_eq!(1, state.n_players_connected());
        let connected: Vec<&PlayerState> = state.connected_players().collect();
        assert_eq!(1, connected.len());
        assert_eq!(1, connected[0].id)
    }
}
