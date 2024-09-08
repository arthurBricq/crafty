use crate::actions;
use crate::server::server_state::PlayerState;
use crate::world;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::world::World;
use crate::entity::entity::EntityKind;

const MONSTER1_SPEED: f32 = 0.1;
const MONSTER1_ROTATION_SPEED: f32 = 0.02;

#[derive(Clone)]
/// Action that the monster can realize
pub enum MonsterAction {
    Forward,
    LeftRot,
    RightRot,
    Attack,
    Jump,
    Idle   
}

/// This trait implement a state machine for the internal logic of monster
pub trait TransitionState {
    /// Return the monster's action
    fn action(&self) -> MonsterAction;
    /// Change the internal state of the machine 
    fn update(&mut self, dt: f32, position: &Position, world: &World, player_list: Vec<PlayerState>);
    fn new() -> Self;
}

/// Contain the data of a monster
pub struct Monster<T> {
    id: usize,
    entity_type: EntityKind,
    position: Position,
    transition: T
}

impl<T> Monster<T> where T: TransitionState {
    pub fn new(id: usize, entity_type: EntityKind, position: Position) -> Self {      
        Self { 
            id,
            entity_type,
            position,
            transition: TransitionState::new(),
        }
    }

    /// Update the state of the monster and do an action (move, attack)
    pub fn update(&mut self, world: &World, dt: f32, player_list: Vec<PlayerState>) {
        // Update the internal state of transition
        self.transition.update(dt, &self.position, world, player_list);

        // Apply the action return by transition
        self.apply_action(self.transition.action());

    }

    /// Apply the action of the monster
    fn apply_action(&mut self, action: MonsterAction) {
        match action {
            MonsterAction::Forward => self.position.set_position(self.position.pos() +  Vector3::new(MONSTER1_SPEED, 0., 0.).rotation_y(self.position.yaw())),
            MonsterAction::LeftRot => self.position.rotate_yaw(MONSTER1_ROTATION_SPEED),
            MonsterAction::RightRot => self.position.rotate_yaw(-MONSTER1_ROTATION_SPEED),
            _ => ()
        }
    }
        
    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
    
    pub fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod test {
    use crate::{entity::entity::EntityKind, primitives::vector::Vector3};
    use crate::primitives::position::Position;
    use super::{MONSTER1_SPEED,MONSTER1_ROTATION_SPEED};

    use super::Monster;
    use crate::entity::walker_in_circle::WalkInCercle;



    #[test]
    fn test_apply_action() {
        let mut monster = Monster::<WalkInCercle>::new(0, EntityKind::Monster1, Position::empty());
        monster.apply_action(super::MonsterAction::Forward);
        assert_eq!(monster.position().pos(), Vector3::new(MONSTER1_SPEED, 0., 0.));
        assert_eq!(monster.position().yaw(), 0.);
        assert_eq!(monster.position().pitch(), 0.);

        let mut monster = Monster::<WalkInCercle>::new(0, EntityKind::Monster1, Position::empty());
        monster.apply_action(super::MonsterAction::LeftRot);
        assert_eq!(monster.position().pos(), Vector3::new(0., 0., 0.));
        assert_eq!(monster.position().yaw(), MONSTER1_ROTATION_SPEED);
        assert_eq!(monster.position().pitch(), 0.);

        let mut monster = Monster::<WalkInCercle>::new(0, EntityKind::Monster1, Position::empty());
        monster.apply_action(super::MonsterAction::RightRot);
        assert_eq!(monster.position().pos(), Vector3::new(0., 0., 0.));
        assert_eq!(monster.position().yaw(), -MONSTER1_ROTATION_SPEED);
        assert_eq!(monster.position().pitch(), 0.);
    }
}
