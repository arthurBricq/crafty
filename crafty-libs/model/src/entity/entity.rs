use crate::collision::aabb::AABB;
use crate::entity::humanoid;
use crate::entity::humanoid::humanoid_aabb;
use primitives::opengl::entity::EntityCube;
use primitives::position::Position;

#[derive(Debug, PartialEq, Clone)]
/// Enum for the different types of entity
pub enum EntityKind {
    Player,
    Monster1,
    Monster2,
}

impl EntityKind {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Player => 0,
            Self::Monster1 => 1,
            Self::Monster2 => 2,
        }
    }

    pub fn from_u8(entity_code: u8) -> Self {
        match entity_code {
            0 => Self::Player,
            1 => Self::Monster1,
            2 => Self::Monster2,
            _ => Self::Monster1,
        }
    }

    pub fn is_player(&self) -> bool {
        match self {
            Self::Player => true,
            _ => false,
        }
    }
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
            EntityKind::Player | EntityKind::Monster1 | EntityKind::Monster2 => {
                humanoid_aabb(&self.position)
            }
        }
    }
}
