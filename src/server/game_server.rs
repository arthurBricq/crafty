use crate::actions::Action;
use crate::entity::entity::EntityKind;
use crate::network::server_update::ServerUpdate;
use crate::network::server_update::ServerUpdate::{Attack, LoggedIn, RegisterEntity, SendAction, UpdatePosition, RemoveEntity};
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::server::monster_manager::MonsterManager;
use crate::server::server_state::ServerState;
use crate::server::world_dispatcher::WorldDispatcher;
use crate::world::World;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::attack::EntityAttack;
use std::time::{Duration, Instant};

/// Main function of the thread in charge of entities
pub fn handle_entity_thread(server: Arc<Mutex<GameServer>>) {
    let sleep_time = Duration::from_millis(5);

    server.lock().unwrap().monster_manager
        .spawn_new_monster(Position::new(Vector3::new(-5., 16., 0.), 0., 0.), EntityKind::Monster1);

    server.lock().unwrap().monster_manager
        .spawn_new_monster(Position::new(Vector3::new(5., 16., 0.), 0., 0.), EntityKind::Monster1);
    
    // server.lock().unwrap().monster_manager
    //     .spawn_new_monster(Position::new(Vector3::new(10., 16., 0.), 0., 0.), EntityKind::Monster1);

    let mut t = Instant::now();
    let mut dt = 0.;
    loop {
        dt = t.elapsed().as_secs_f32();
        t = Instant::now();
        
        let player_list = server.lock().unwrap().state.connected_players().cloned().collect();
        server.lock().unwrap().monster_manager.step(dt, &player_list);
        server.lock().unwrap().update_buffers();
        std::thread::sleep(sleep_time);
    }
}

/// The GameServer is the model of the server
pub struct GameServer {
    /// The full world
    world: Arc<Mutex<World>>,

    /// In charge of telling which chunks must be loaded by which player
    world_dispatcher: WorldDispatcher,

    /// Buffer of updates to be sent to each player
    server_updates_buffer: HashMap<usize, Vec<ServerUpdate>>,

    /// In charge of handling the entities
    monster_manager: MonsterManager,
    
    /// Internal state of the server (expect the entities)
    state: ServerState
}

impl GameServer {
    pub fn new(world: World) -> Self {
        let ref_to_world = Arc::new(Mutex::new(world));
        Self {
            world: Arc::clone(&ref_to_world),
            world_dispatcher: WorldDispatcher::new(),
            server_updates_buffer: HashMap::new(),
            monster_manager: MonsterManager::new(ref_to_world),
            state: ServerState::new()
        }
    }

    /// Logins a new player into the server
    /// Returns the ID of the registered player
    pub fn login(&mut self, name: String) -> usize {
        // Create the new ID
        let player = self.state.login(name.clone());
        println!("[SERVER] New player registered: {name} (ID={}, pos={:?})", player.id, player.pos);
        println!("Connected players: {}", self.state.n_players_connected());
        
        // Create a new buffer of updates for this client, 
        let mut initial_updates = vec![LoggedIn(player.id as u8, player.pos.clone())];

        // Initialize it directly with a LoggedIn message and the position of the other players
        for (i, connected) in self.state.connected_players().enumerate() {
            if connected.id != player.id {
                initial_updates.push(RegisterEntity(i as u8, EntityKind::Player, player.pos.clone()))
            }
        }
        
        let monster_entry = self.monster_manager.get_monsters();
        initial_updates.append(&mut monster_entry.clone());

        self.server_updates_buffer.insert(player.id, initial_updates);

        // Register the player in the dispatcher
        self.world_dispatcher.register_player(player.id);

        // Register the new player to other players of the game.
        for player in self.state.connected_players() {
            self.server_updates_buffer.get_mut(&player.id).unwrap()
                .push(RegisterEntity(player.id as u8, EntityKind::Player, player.pos.clone()));
        }

        player.id
    }

    pub fn logout(&mut self, id: usize) {
        // The world dispatcher must be informed that this client loose all of its chunks
        self.state.logout(id);
        self.world_dispatcher.logout(id);
    }

    // Implementation of the 'callbacks': entry points of the server

    /// Called when receiving the position of a new player
    pub fn on_new_position_update(&mut self, player_id: usize, position: Position) {
        // Update the world dispatcher. to compute if the player needs to be sent new chunks
        if let Some((chunks_to_send, _chunks_to_delete)) = self.world_dispatcher.update_position(player_id, (position.x(), position.z())) {
            for corner in chunks_to_send {
                // Find the correct chunk
                if let Some(to_send) = self.world.lock().unwrap().get_chunk(corner) {
                    self.server_updates_buffer.get_mut(&player_id).unwrap().push(ServerUpdate::LoadChunk(to_send))
                } else {
                    // TODO generate chunk !!!
                }
            }
        }

        // Update other players
        for player in self.state.connected_players() {
            if player.id != player_id {
                self.server_updates_buffer.get_mut(&player.id).unwrap().push(UpdatePosition(player_id as u8, position.clone()))
            }
        }

        // Update internal state
        self.state.set_player_pos(player_id, position.clone());
    }

    pub fn on_new_action(&mut self, player_id: usize, action: Action) {
        // Edit the world of the server
        self.world.lock().unwrap().apply_action(&action);

        // Forward the action to all the other connected players
        for player in self.state.connected_players() {
            if player.id != player_id {
                self.server_updates_buffer.get_mut(&player.id).unwrap().push(SendAction(action.clone()))
            }
        }

    }

    pub fn on_new_attack(&mut self, attack: EntityAttack) {
        println!("Attacked received: {attack:?}");
        let victim = attack.victim_id() as usize;

        // Communicate the attack to victim, if the victim is a player.
        for player in self.state.connected_players() {
            if player.id == victim {
                if let Some(buf) = self.server_updates_buffer.get_mut(&player.id) {
                    buf.push(Attack(attack));
                    return;
                }
            }
        }

        // If we arrive here, it means the victim is not one of the connected player.
        // Therefore, it must be a monster.

        // In this case, we are attacking a monster. Let's kill the monster and forward the update to all other players.
        self.monster_manager.remove_monster(victim as usize);

        // Forward to the other players that the monster was killed.
        for player in self.state.connected_players() {
            self.server_updates_buffer.get_mut(&player.id).unwrap().push(RemoveEntity(victim as u32));
        }

    }

    /// Returns the list of updates that the server sends to the client.
    pub fn consume_updates(&mut self, player_id: usize) -> Vec<ServerUpdate> {
        self.server_updates_buffer.insert(player_id, Vec::new()).unwrap()
    }

    fn update_buffers(&mut self) {
        // Add to these updates the ones that the entity manager also provides
        let monster_updates = self.monster_manager.get_server_updates().clone();
        self.server_updates_buffer.iter_mut().for_each(|(_, buffer)| buffer.append(&mut monster_updates.clone()));
    }
}


#[cfg(test)]
mod tests {
    use crate::attack::EntityAttack;
    use crate::network::server_update::ServerUpdate;
    use crate::server::game_server::GameServer;
    use crate::world::World;

    #[test]
    fn test_two_clients_connecting() {
        // Create a server with an empty world
        let mut server = GameServer::new(World::empty());

        // first client logins
        let id1 = server.login("arthur".to_string());

        // We expect 1 update: the login message
        let updates = server.consume_updates(id1);
        assert_eq!(1, updates.len());
        assert!(matches!(updates[0], ServerUpdate::LoggedIn(_, _)));

        // Once the update has been consumed, there is nothing anymore to be sent
        assert_eq!(0, server.consume_updates(id1).len());

        // Second client logins
        let id2 = server.login("johan".to_string());

        // We expect 1 new update for the first player: the register message
        let updates = server.consume_updates(id1);
        assert_eq!(1, updates.len());
        assert!(matches!(updates[0], ServerUpdate::RegisterEntity(_, _, _)));

        // The second player must have 2 messages: LoggedIn and Register
        let updates = server.consume_updates(id2);
        assert_eq!(2, updates.len());
        assert!(matches!(updates[0], ServerUpdate::LoggedIn(_, _)));
        assert!(matches!(updates[1], ServerUpdate::RegisterEntity(_, _, _)));
    }
    
    #[test]
    fn test_attack_broacasting() {
        
        // Create a server with an empty world
        let mut server = GameServer::new(World::empty());

        let id1 = server.login("arthur".to_string());
        let id2 = server.login("johan".to_string());
        let id3 = server.login("arnaud".to_string());
        
        // consumes all updates
        server.consume_updates(id1);
        server.consume_updates(id2);
        server.consume_updates(id3);
        
        // johan attacks arnaud
        server.on_new_attack(EntityAttack::new(id3 as u8));
        
        // only arnaud is supposed to receive a message
        assert_eq!(0, server.consume_updates(id1).len());
        assert_eq!(0, server.consume_updates(id2).len());
        assert_eq!(1, server.consume_updates(id3).len());
        
    }

}