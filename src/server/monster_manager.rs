use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::entity::entity::{Entity, EntityKind};
use crate::network::server_update::ServerUpdate;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::world::World;

struct Monster(Entity);

impl Monster {
    
}

pub struct MonsterManager {
    world: Arc<Mutex<World>>,
    monsters: Vec<Monster>
}

impl MonsterManager {
    pub fn new(world: Arc<Mutex<World>>) -> Self {
        Self {
            world,
            monsters: Vec::new()
        }
    }

    pub fn spawn_new_monster(&mut self, at: Vector3, kind: EntityKind) {
        let id = self.monsters.len();
        self.monsters.push(Monster(Entity::new(id, kind, Position::from_pos(at))))
    }

    pub fn step(&mut self, dt: Duration) {
        // println!("info from world ? {}", self.world.lock().unwrap().)
    }
    
    pub fn get_server_updates(&mut self, from: Position) -> Vec<ServerUpdate> {
        vec![]
    }
}