use crate::entity::{self, entity::EntityKind};

#[derive(PartialEq, Debug, Clone)]
pub struct EntityAttack{    
    attacked: u8,
    kind: EntityKind,
    strength: u8
}

impl EntityAttack {
    pub fn new(attacked: u8, kind: EntityKind) -> Self {
        Self {
            attacked,
            kind,
            strength: 1
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.attacked, self.kind.to_u8(), self.strength]
    }

    pub fn from_bytes(bytes_to_parse: &[u8]) -> Self {
        Self {
            attacked: bytes_to_parse[0],
            kind: EntityKind::from_u8(bytes_to_parse[1]),
            strength: bytes_to_parse[2]
        }
    }

    pub fn set_strength(&mut self, strength: u8) {
        self.strength = strength;
    }

    pub fn strength(&self) -> u8 {
        self.strength
    }

    pub fn victim_id(&self) -> u8 {
        self.attacked
    }

    pub fn entity_kind(&self) -> &EntityKind {
        &self.kind
    }
}