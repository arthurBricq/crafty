use crate::attack::EntityAttack;
use crate::cube::Cube;
use crate::entity::entity::{Entity, EntityKind};
use crate::graphics::entity::EntityCube;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use std::collections::HashMap;

/// Contain all the entities
pub struct EntityManager {
    entities: HashMap<u8, Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    /// Register another player, provided its id and initial position
    pub fn register_new_entity(&mut self, id: u8, entity_kind: EntityKind, pos: Position) {
        println!("New player has joined the game: {id}");
        let entity = Entity::new(id as usize, entity_kind, pos.clone());
        self.entities.insert(id, entity);
    }

    /// Remove an entity from the Manager
    pub fn remove_entity(&mut self, id: u8) {
        if !self.entities.contains_key(&id) {
            panic!("Trying to remove an id that do not exist")
        }
        self.entities.remove(&id);
    }

    pub fn set_position(&mut self, id: u8, position: Position) {
        self.entities
            .get_mut(&id)
            .map(|entity| entity.set_position(position));
    }

    /// Returns the list of OpenGL attributes to be rendered
    pub fn get_opengl_entities(&self) -> Vec<EntityCube> {
        self.entities
            .iter()
            .map(|(_, entity)| entity.get_opengl_entities())
            .collect::<Vec<Vec<EntityCube>>>()
            .concat()
    }

    pub fn attack(&self, position: Vector3, direction: Vector3) -> Option<EntityAttack> {
        if let Some((id, _)) = self
            .entities
            .iter()
            .map(|(id, entity)| (id, entity.aabb().faces()))
            .find(|(id, faces)| {
                Cube::intersection_with_faces(&faces, position, direction).is_some()
            })
        {
            println!("Player {id} was hit !");
            return Some(EntityAttack::new(*id));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::attack::EntityAttack;
    use crate::entity::entity::EntityKind;
    use crate::entity::entity_manager::EntityManager;
    use crate::primitives::position::Position;
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_basic_functionality() {
        let mut mgr = EntityManager::new();
        assert_eq!(0, mgr.get_opengl_entities().len());

        mgr.register_new_entity(
            2,
            EntityKind::Monster2,
            Position::from_pos(Vector3::unit_x()),
        );
        assert_eq!(6, mgr.get_opengl_entities().len());

        mgr.register_new_entity(
            3,
            EntityKind::Monster1,
            Position::from_pos(Vector3::unit_x()),
        );
        assert_eq!(12, mgr.get_opengl_entities().len());
    }

    #[test]
    fn test_attack() {
        let mut mgr = EntityManager::new();

        // Add a player at the origin
        mgr.register_new_entity(0, EntityKind::Player, Position::from_pos(Vector3::empty()));

        assert_eq!(
            Some(EntityAttack::new(0)),
            mgr.attack(Vector3::unit_x(), Vector3::unit_x().opposite())
        );
        assert_eq!(None, mgr.attack(Vector3::unit_x(), Vector3::unit_x()));
        assert_eq!(None, mgr.attack(Vector3::unit_x(), Vector3::unit_y()));
        assert_eq!(None, mgr.attack(Vector3::unit_x(), Vector3::unit_z()));
    }

    #[test]
    #[should_panic]
    fn test_remove() {
        let mut mgr = EntityManager::new();
        assert_eq!(0, mgr.get_opengl_entities().len());

        mgr.register_new_entity(
            2,
            EntityKind::Monster2,
            Position::from_pos(Vector3::unit_x()),
        );
        assert_eq!(6, mgr.get_opengl_entities().len());

        mgr.remove_entity(2);
        assert_eq!(0, mgr.get_opengl_entities().len());

        mgr.remove_entity(5);
    }
}
