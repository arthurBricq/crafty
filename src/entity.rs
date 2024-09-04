use crate::graphics::entity::EntityCube;
use crate::humanoid;
use crate::vector::Vector3;


#[derive(Debug, PartialEq)]
/// Enum for the different types of entity
pub enum EntityKind {
    Player
}

/// Contain the data of an entity
pub struct Entity {
    id: usize,
    entity_type: EntityKind,
    position: Vector3,
    orientation: [f32;2],
}

impl Entity {
    pub fn new(id: usize, entity_type: EntityKind, position: Vector3, orientation: [f32;2]) -> Self {
        Self { 
            id,
            entity_type,
            position,
            orientation,
        }
    }
    
    pub fn set_position(&mut self, position: Vector3) {
        self.position = position;
    }
    
    pub fn set_orientation(&mut self, orientation: [f32;2]) {
        self.orientation = orientation;
    }
    
    pub fn position(&self) -> &Vector3 {
        &self.position
    }
    
    pub fn orientation(&self) -> [f32; 2] {
        self.orientation
    }
        
    pub fn id(&self) -> usize {
        self.id
    }
        
    pub fn entity_type(&self) -> &EntityKind {
        &self.entity_type
    }

    /// Draw the entity and return a Vec of EntityCube
    pub fn draw_entity(&mut self) -> Vec<EntityCube> {

        let ent = match self.entity_type {
            EntityKind::Player => {
                humanoid::draw( self.position, self.orientation)
            }
        };

        ent
    }

}

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

#[cfg(test)]
mod tests {
    use crate::{entity::EntityKind, vector::Vector3};
    use super::Entity;

    #[test]
    fn test_instanciating_entity() {
        let entity= Entity::new(2, super::EntityKind::Player, Vector3::new(0., 0., 0.), [0., 1.]);
        assert_eq!( entity.position(), &Vector3::new(0., 0., 0.));
        assert_eq!( entity.orientation(), [0., 1.]);
        assert_eq!( entity.id(), 2);
        assert_eq!( entity.entity_type(), &EntityKind::Player);
    }
    #[test]
    fn test_set_entity() {
        let mut entity= Entity::new(2, super::EntityKind::Player, Vector3::new(0., 0., 0.), [0., 1.]);
        entity.set_orientation([1.0, 0.0]);
        entity.set_position(Vector3::new(2., 2., 2.));
        assert_eq!( entity.position(), &Vector3::new(2., 2., 2.));
        assert_eq!( entity.orientation(), [1.0, 0.0]);
    }
}