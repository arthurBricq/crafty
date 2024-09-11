use crate::aabb::AABB;
use crate::graphics::entity::EntityCube;
use crate::entity::humanoid;
use crate::entity::humanoid::humanoid_aabb;
use crate::primitives::position::Position;

#[derive(Debug, PartialEq)]
/// Enum for the different types of entity
pub enum EntityKind {
    Player,
    Monster1,
    Monster2
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
            EntityKind::Player => humanoid::get_opengl_entities(self.position.clone(), 0),
            EntityKind::Monster1 => humanoid::get_opengl_entities(self.position.clone(), 1),
            EntityKind::Monster2 => humanoid::get_opengl_entities(self.position.clone(), 2),
        }
    }

    pub fn aabb(&self) -> AABB {
        match self.entity_type {
            EntityKind::Player | EntityKind::Monster1 | EntityKind::Monster2 => humanoid_aabb(&self.position)
        
        }
    }
}
