use crate::graphics::entity::EntityCube;
use crate::entity::humanoid;
use crate::primitives::vector::Vector3;


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
    pub fn get_opengl_entities(&self) -> Vec<EntityCube> {

        let ent = match self.entity_type {
            EntityKind::Player => {
                humanoid::get_opengl_entities( self.position, self.orientation)
            }
        };

        ent
    }

}


#[cfg(test)]
mod tests {
    use crate::entity::entity::EntityKind;
    use crate::primitives::vector::Vector3;
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