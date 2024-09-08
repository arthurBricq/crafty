use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::entity::walker_in_circle::WalkInCercle;
use crate::entity::entity::EntityKind;
use crate::entity::monster::Monster;
use crate::network::server_update::ServerUpdate;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::world::World;

use super::server_state::PlayerState;

pub struct MonsterManager {
    world: Arc<Mutex<World>>,
    monsters: Vec<Monster<WalkInCercle>>,
    buffer_update: Vec<ServerUpdate>,
}

impl MonsterManager {
    pub fn new(world: Arc<Mutex<World>>) -> Self {
        Self {
            world,
            monsters: Vec::new(),
            buffer_update: Vec::new()
        }
    }

    pub fn spawn_new_monster(&mut self, at: Vector3, kind: EntityKind) -> usize {
        let id = self.generate_id();
        let pos = Position::from_pos(at);
        self.monsters.push(Monster::new(id, kind, pos.clone()));
        // Inform the player that a new entity has spawn
        self.buffer_update.push(ServerUpdate::RegisterEntity(id as u8, pos));
        id
    }

    // TODO add a message to client to remove an entity !!
    /// Remove a monster with id id from the Manager
    pub fn remove_monster(&mut self, id: usize) {
        for index in 0..self.monsters.len() {
            if self.monsters[index].id() == id {
                self.monsters.swap_remove(index);
                return;
            }
        }
        panic!("Try to remove monster with id {}, not found ", id)
    }

    /// Ask the monster to move
    pub fn step(&mut self, dt: Duration, player_list: Vec<PlayerState>) {
        self.monsters.iter_mut()
        .for_each(|monster | {
            monster.update(&self.world.lock().unwrap(), dt.as_secs_f32(), player_list.clone());
            // Inform the players that the monster has moved
            self.buffer_update.push(ServerUpdate::UpdatePosition(monster.id() as u8, monster.position().clone()));
        } );
    }
    
    /// Return the updated position of the monsters
    pub fn get_server_updates(&mut self) -> Vec<ServerUpdate> {
        // TODO update the player only if monster is close enought
        let buffer = self.buffer_update.clone();
        self.buffer_update = Vec::new();
        buffer
    }

    /// Return the ServerUpdate with all entities
    /// Used to register all the monster to a ew player
    pub fn get_monsters(&self) -> Vec<ServerUpdate> {
        let mut vec_update = Vec::new();
        for monster in &self.monsters {
            vec_update.push(ServerUpdate::RegisterEntity(monster.id() as u8, monster.position().clone()));
        }
        vec_update
    }

    fn generate_id(&self) -> usize {
        50 + self.monsters.len()
    }

}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use crate::primitives::vector::Vector3;
    use crate::world::World;
    use crate::network::server_update::ServerUpdate;
    use super::MonsterManager;
    use crate::primitives::position::{self, Position};

    #[test]
    fn test_add_monster() {
        let world = Arc::new(Mutex::new(World::empty()));
        let mut monster_manager = MonsterManager::new(world);

        let id0 = monster_manager.spawn_new_monster(Vector3::new(0., 0., 0.), crate::entity::entity::EntityKind::Monster1);
    
        let monsters_update = monster_manager.get_monsters();
        assert_eq!(monsters_update.len(), 1);

        let updates = monster_manager.get_server_updates(Position::new(Vector3::new(0., 0., 0.), 0., 0.));
        assert_eq!(updates.len(), 1);
        
        let id1 = monster_manager.spawn_new_monster(Vector3::new(0., 0., 0.), crate::entity::entity::EntityKind::Monster1);
        
        let monsters_update = monster_manager.get_monsters();
        assert_eq!(monsters_update.len(), 2);

        let updates = monster_manager.get_server_updates(Position::new(Vector3::new(0., 0., 0.), 0., 0.));
        assert_eq!(updates.len(), 1);
    }
    
    #[test]
    fn test_rm_monster() {
        let world = Arc::new(Mutex::new(World::empty()));
        let mut monster_manager = MonsterManager::new(world);

        let id = monster_manager.spawn_new_monster(Vector3::new(0., 0., 0.), crate::entity::entity::EntityKind::Monster1);
    
        monster_manager.get_server_updates(Position::new(Vector3::new(0., 0., 0.), 0., 0.));

        monster_manager.remove_monster(id);
        let monsters_update =monster_manager.get_monsters();
        assert_eq!(monsters_update.len(), 0);

        let updates = monster_manager.get_server_updates(Position::new(Vector3::new(0., 0., 0.), 0., 0.));
        // When adding a message to client to remove the monster, set 0 to 1
        assert_eq!(updates.len(), 0);

    }

    #[test]
    fn test_get_monsters() {
        let world = Arc::new(Mutex::new(World::empty()));
        let mut monster_manager = MonsterManager::new(world);

        let id0 = monster_manager.spawn_new_monster(Vector3::new(0., 0., 0.), crate::entity::entity::EntityKind::Monster1);
        let monsters_update = monster_manager.get_monsters();
        let position0 = Position::new(Vector3::new(0., 0., 0.), 0., 0.);
        match &monsters_update[0] {
            ServerUpdate::RegisterEntity(id, position) => {
                assert_eq!(id0, *id as usize);
                assert_eq!(position0, *position)
            },
            _ => assert!(false)
        }
    }


}