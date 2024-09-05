use std::collections::HashMap;
use crate::entity::entity::{Entity, EntityKind};
use crate::primitives::vector::Vector3;
use crate::graphics::entity::EntityCube;

/// Contain all the entities
pub struct EntityManager {
    entities: HashMap<u8, Entity>
}

impl EntityManager {
    pub fn new() -> Self {
        Self { 
            entities: HashMap::new(),
        }
    }

    /// Register another player, provided its id and initial position
    pub fn register_new_player(&mut self, id: u8, pos: Vector3) {
        println!("New player has joined the game: {id}");
        let entity = Entity::new(id as usize, EntityKind::Player, pos, [0.,0.]);
        self.entities.insert(id, entity);
    }

    pub fn set_position(&mut self, id: u8, position: Vector3) {
        println!("Updating pos for player: {id}");
        self.entities.get_mut(&id).map(|entity| entity.set_position(position));
    }

    pub fn set_orientation(&mut self, id: u8, orientation: [f32; 2]) {
        self.entities.get_mut(&id).map(|entity| entity.set_orientation(orientation));
    }

    /// Returns the list of OpenGL attributes to be rendered
    pub fn get_opengl_entities (&self) -> Vec<EntityCube> {
        self.entities
            .iter()
            .map(|(_, entity)| entity.get_opengl_entities())
            .collect::<Vec<Vec<EntityCube>>>()
            .concat()
    }

}

#[cfg(test)]
mod tests {
    use crate::entity::entity_manager::EntityManager;
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_basic_functionality() {
        let mut mgr = EntityManager::new();
        assert_eq!(0, mgr.get_opengl_entities().len());

        mgr.register_new_player(2, Vector3::unit_x());
        assert_eq!(6, mgr.get_opengl_entities().len());

        mgr.register_new_player(4, Vector3::unit_x());
        assert_eq!(12, mgr.get_opengl_entities().len());
    }
}