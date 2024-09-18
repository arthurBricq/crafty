use std::sync::{Arc, Mutex};

use crate::entity::chaser::Chaser;
use crate::entity::entity::EntityKind;
use crate::entity::monster::Monster;
use crate::network::server_update::ServerUpdate;
use crate::primitives::position::Position;
use crate::world::World;

use super::server_state::PlayerState;

pub struct MonsterManager {
    world: Arc<Mutex<World>>,
    monsters: Vec<Monster<Chaser>>,
    buffer_update: Vec<ServerUpdate>,
}

impl MonsterManager {
    pub fn new(world: Arc<Mutex<World>>) -> Self {
        Self {
            world,
            monsters: Vec::new(),
            buffer_update: Vec::new(),
        }
    }

    pub fn spawn_new_monster(&mut self, pos: Position, kind: EntityKind) -> usize {
        let id = self.generate_id();
        self.monsters.push(Monster::new(id, kind.clone(), pos.clone()));
        // Inform the player that a new entity has spawn
        self.buffer_update.push(ServerUpdate::RegisterEntity(id as u8, kind, pos));
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
    }

    /// Ask the monster to move
    pub fn step(&mut self, dt: f32, players: &Vec<PlayerState>) {
        self.monsters.iter_mut()
            .for_each(|monster| {
                monster.update(&self.world.lock().unwrap(), dt, players);
                // Inform the players that the monster has moved
                self.buffer_update.push(ServerUpdate::UpdatePosition(monster.id() as u8, monster.position().clone()));
            });
    }

    /// Return the updated position of the monsters
    pub fn get_server_updates(&mut self) -> Vec<ServerUpdate> {
        // TODO update the player only if monster is close enought
        let buffer = self.buffer_update.clone();
        self.buffer_update = Vec::new();
        buffer
    }

    /// Return the ServerUpdate with all entities
    /// Used to register all the monster to a new player
    pub fn get_monsters(&self) -> Vec<ServerUpdate> {
        let mut vec_update = Vec::new();
        for monster in &self.monsters {
            vec_update.push(ServerUpdate::RegisterEntity(monster.id() as u8, monster.entity_type().clone(), monster.position().clone()));
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

    use crate::primitives::vector::Vector3;
    use crate::world::World;
    use crate::network::server_update::ServerUpdate;
    use super::MonsterManager;
    use crate::primitives::position::Position;
    use crate::entity::entity::EntityKind;

    #[test]
    fn test_add_monster() {
        let world = Arc::new(Mutex::new(World::empty()));
        let mut monster_manager = MonsterManager::new(world);
        let pos = Position::new(Vector3::empty(), 0., 0.);

        monster_manager.spawn_new_monster(pos.clone(), EntityKind::Monster1);

        let monsters_update = monster_manager.get_monsters();
        assert_eq!(monsters_update.len(), 1);

        let updates = monster_manager.get_server_updates();
        assert_eq!(updates.len(), 1);

        monster_manager.spawn_new_monster(pos, EntityKind::Monster1);

        let monsters_update = monster_manager.get_monsters();
        assert_eq!(monsters_update.len(), 2);

        let updates = monster_manager.get_server_updates();
        assert_eq!(updates.len(), 1);
    }

    #[test]
    fn test_rm_monster() {
        let world = Arc::new(Mutex::new(World::empty()));
        let mut monster_manager = MonsterManager::new(world);
        let pos = Position::new(Vector3::empty(), 0., 0.);

        let id = monster_manager.spawn_new_monster(pos.clone(), crate::entity::entity::EntityKind::Monster1);

        monster_manager.get_server_updates();

        monster_manager.remove_monster(id);
        let monsters_update = monster_manager.get_monsters();
        assert_eq!(monsters_update.len(), 0);

        let updates = monster_manager.get_server_updates();
        // When adding a message to client to remove the monster, set 0 to 1
        assert_eq!(updates.len(), 0);
    }

    #[test]
    fn test_get_monsters() {
        let world = Arc::new(Mutex::new(World::empty()));
        let mut monster_manager = MonsterManager::new(world);
        let position0 = Position::new(Vector3::new(0., 0., 0.), 0., 0.);
        let entity_type0 = EntityKind::Monster1;

        let id0 = monster_manager.spawn_new_monster(position0.clone(), entity_type0.clone());
        let monsters_update = monster_manager.get_monsters();
        match &monsters_update[0] {
            ServerUpdate::RegisterEntity(id, entity_pos1, position1) => {
                assert_eq!(id0, *id as usize);
                assert_eq!(position0, *position1);
                assert_eq!(entity_type0, *entity_pos1);
            }
            _ => assert!(false)
        }
    }
}