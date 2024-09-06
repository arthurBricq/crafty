use crate::graphics::entity::EntityCube;
use crate::entity::humanoid;
use crate::primitives::position::Position;



#[derive(Debug, PartialEq)]
/// Enum for the different types of entity
pub enum EntityKind {
    Player
}

/// Contain the data of an entity
pub struct Entity {
    id: usize,
    entity_type: EntityKind,
    position: Position,
}

impl Entity {
    pub fn new(id: usize, entity_type: EntityKind, position: Position) -> Self {
        Self { 
            id,
            entity_type,
            position,
        }
    }
    
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
    
    pub fn id(&self) -> usize {
        self.id
    }
        
    pub fn entity_type(&self) -> &EntityKind {
        &self.entity_type
    }

    /// Draw the entity and return a Vec of EntityCube
    pub fn get_opengl_entities(&self) -> Vec<EntityCube> {
        match self.entity_type {
            EntityKind::Player => humanoid::get_opengl_entities(self.position.clone())
        }
    }

}
