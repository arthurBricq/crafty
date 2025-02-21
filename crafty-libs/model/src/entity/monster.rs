use crate::collision::collidable::{Collidable, CollisionData};
use crate::entity::entity::EntityKind;
use crate::game::attack::EntityAttack;
use crate::game::player::{GRAVITY_ACCELERATION_VECTOR, JUMP_VELOCITY, PLAYER_MARGIN};
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::server::server_state::PlayerState;
use crate::world::world::World;
use super::humanoid::humanoid_aabb;
const MONSTER1_SPEED: f32 = 2.;
const MONSTER1_ROTATION_SPEED: f32 = 0.10;

#[derive(Clone)]
/// Action that the monster can realize
pub enum MonsterAction {
    Forward,
    LeftRot,
    RightRot,
    Attack(usize),
    Jump,
    Idle,
}

/// This trait implement a state machine for the internal logic of monster
pub trait TransitionState {
    /// Return the monster's action
    fn action(&self) -> MonsterAction;
    /// Change the internal state of the machine
    fn update(
        &mut self,
        dt: f32,
        position: &Position,
        world: &World,
        player_list: &Vec<PlayerState>,
    );
    fn new() -> Self;
}

/// Contain the data of a monster
pub struct Monster<T> {
    id: usize,
    entity_type: EntityKind,
    position: Position,
    transition: T,
    in_air: bool,
    velocity: Vector3,
    /// If the monster is attacking, if so give the id of the victim
    attack: Option<EntityAttack>,
}

impl<T> Monster<T>
where
    T: TransitionState,
{
    pub fn new(id: usize, entity_type: EntityKind, position: Position) -> Self {
        Self {
            id,
            entity_type,
            position,
            transition: TransitionState::new(),
            in_air: true,
            velocity: Vector3::empty(),
            attack: None,
        }
    }

    /// Update the state of the monster and do an action (move, attack)
    pub fn update(&mut self, world: &World, dt: f32, player_list: &Vec<PlayerState>) {
        // Update the internal state of transition
        self.transition
            .update(dt, &self.position, world, player_list);

        // Apply the action return by transition
        self.apply_action(self.transition.action(), dt, world);
    }

    /// Apply the action of the monster
    fn apply_action(&mut self, action: MonsterAction, mut dt: f32, world: &World) {
        self.attack = None;
        match action {
            MonsterAction::Forward => {
                let velocity_hor = self.position.ground_direction_forward() * MONSTER1_SPEED;
                self.velocity[0] = velocity_hor[0];
                self.velocity[2] = velocity_hor[2];
            }
            MonsterAction::LeftRot => self.position.rotate_yaw(MONSTER1_ROTATION_SPEED),
            MonsterAction::RightRot => self.position.rotate_yaw(-MONSTER1_ROTATION_SPEED),
            MonsterAction::Jump => self.jump(),
            MonsterAction::Idle => {
                self.velocity[0] = 0.;
                self.velocity[2] = 0.;
            }
            MonsterAction::Attack(attacked) => {
                self.attack = Some(EntityAttack::new(attacked as u8));
                self.velocity[0] = 0.;
                self.velocity[2] = 0.;
            }
        }

        if self.in_air {
            self.velocity += GRAVITY_ACCELERATION_VECTOR * dt;
        }

        loop {
            dt = dt - self.move_with_collision(dt, world);
            if dt <= 0. {
                break;
            }
        }

        // update in_air
        let displacement = Vector3::new(0., -2.0 * PLAYER_MARGIN, 0.);
        self.in_air = !world.collides(&humanoid_aabb(&(&self.position + displacement)));
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

    pub fn attack(&self) -> &Option<EntityAttack> {
        &self.attack
    }

    /// Integrate the velocity to move the camera, with collision. Returns the
    /// dt (in seconds), which can be smaller than `dt` if there is a collision.
    fn move_with_collision(&mut self, dt: f32, world: &World) -> f32 {
        let target = humanoid_aabb(&(&self.position + self.velocity * dt));

        let collision = world
            .collision_time(
                &self.position,
                &humanoid_aabb(&self.position),
                &target,
                &self.velocity,
            )
            .unwrap_or(CollisionData {
                time: f32::MAX,
                normal: Vector3::empty(),
            });

        if collision.time >= dt {
            // TO DO : do we also want a margin here ???????????

            // can move straight away
            self.position += self.velocity * dt;

            dt
        } else {
            // The margin is between the player and the block we colide with
            // need the projection of velocity onto the normal
            let mut dtmargin: f32 = 0.0;
            if self.velocity.norm() >= 1e-10 {
                dtmargin = PLAYER_MARGIN / collision.normal.dot(&self.velocity).abs();
            }
            // we want to put a margin, to avoid collision even with floats rounding
            self.position += self.velocity * (collision.time - dtmargin);

            // remove component of velocity along the normal
            let vnormal = collision.normal * collision.normal.dot(&self.velocity);
            self.velocity = self.velocity - vnormal;

            collision.time
        }
    }

    pub fn jump(&mut self) {
        if !self.in_air {
            self.velocity[1] = JUMP_VELOCITY;
        }
    }
}
