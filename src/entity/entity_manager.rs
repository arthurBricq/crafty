use crate::entity::entity::{Entity, EntityKind};
use crate::graphics::entity::EntityCube;
use crate::primitives::position::Position;
use std::collections::HashMap;
use crate::cube::Cube;
use crate::primitives::vector::Vector3;

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
    pub fn register_new_player(&mut self, id: u8, pos: Position) {
        println!("New player has joined the game: {id}");
        let entity = Entity::new(id as usize, EntityKind::Player, pos);
        self.entities.insert(id, entity);
    }

    pub fn set_position(&mut self, id: u8, position: Position) {
        self.entities.get_mut(&id).map(|entity| entity.set_position(position));
    }

    /// Returns the list of OpenGL attributes to be rendered
    pub fn get_opengl_entities (&self) -> Vec<EntityCube> {
        self.entities
            .iter()
            .map(|(_, entity)| entity.get_opengl_entities())
            .collect::<Vec<Vec<EntityCube>>>()
            .concat()
    }
    
    pub fn attack(&self, position: Vector3, direction: Vector3) {
        println!("Attacking !");
        if let Some((id, _)) = self.entities
            .iter()
            .map(|(id, entity)| (id, entity.aabb().faces())) 
            .find(|(id, faces)| Cube::intersection_with_faces(&faces, position, direction).is_some()) 
        {
            println!("Player {id} was hit !");
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::entity::entity_manager::EntityManager;
    use crate::primitives::position::Position;
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_basic_functionality() {
        let mut mgr = EntityManager::new();
        assert_eq!(0, mgr.get_opengl_entities().len());

        mgr.register_new_player(2, Position::from_pos(Vector3::unit_x()));
        assert_eq!(6, mgr.get_opengl_entities().len());

        mgr.register_new_player(3, Position::from_pos(Vector3::unit_x()));
        assert_eq!(12, mgr.get_opengl_entities().len());
    }
}