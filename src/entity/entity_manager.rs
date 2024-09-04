use crate::entity::entity::{Entity, EntityKind};
use crate::vector::Vector3;
use crate::graphics::entity::EntityCube;

/// Contain all the entities
pub struct EntityManager {
    entity_list: Vec<Entity>,
    n_entity: usize
}

impl EntityManager {
    pub fn empty() -> Self {
        Self { 
            entity_list: Vec::new(),
            n_entity: 0
        }
    }

    pub fn new_entity(&mut self) {
        self.n_entity += 1;
        self.entity_list.push(Entity::new(self.n_entity, EntityKind::Player, Vector3::empty(), [0.,0.]));
    }

    pub fn set_position(&mut self, id: usize, position: Vector3) {
        self.entity_list[id].set_position(position);
    }

    pub fn set_orientation(&mut self, id: usize, orientation: [f32;2]) {
        self.entity_list[id].set_orientation(orientation);
    }

    pub fn position(&self, id: usize) -> &Vector3 {
        self.entity_list[id].position()
    }

    pub fn draw (&mut self) -> Vec<EntityCube> {
        let mut entity_cube = Vec::new();
        for i in 0..self.n_entity {
            entity_cube.append(&mut self.entity_list[i].draw_entity())
        }
        entity_cube
    }


}