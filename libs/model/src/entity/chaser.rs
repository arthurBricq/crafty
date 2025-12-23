use super::monster::{MonsterAction, TransitionState};
use primitives::position::Position;
use primitives::vector::Vector3;
use crate::server::server_state::PlayerState;
use crate::world::world::World;

const CHASING_DISTANCE: f32 = 10.;
const CHASER_ATTACK_COOLDOWN: f32 = 2.; // Time in second before new attack
const ATTACK_RANGE: f32 = 1.8;

/// Internal state of the monster,
/// name are for convinience, they are not forced to a particular action before the action function
#[derive(Clone)]
pub enum MonsterStateEnum {
    Idle,
    Forward,
    TurnLeft,
    TurnRight,
    Attack,
    Jump,
}

pub struct Chaser {
    state: MonsterStateEnum,
    chasing: Option<usize>,
    attack_cooldown: f32,
}

impl TransitionState for Chaser {
    fn action(&self) -> MonsterAction {
        match self.state {
            MonsterStateEnum::Forward => MonsterAction::Forward,
            MonsterStateEnum::TurnLeft => MonsterAction::LeftRot,
            MonsterStateEnum::TurnRight => MonsterAction::RightRot,
            MonsterStateEnum::Attack => MonsterAction::Attack(self.chasing.unwrap()),
            MonsterStateEnum::Jump => MonsterAction::Jump,
            _ => MonsterAction::Idle,
        }
    }
    fn update(
        &mut self,
        dt: f32,
        position: &Position,
        world: &World,
        player_list: &Vec<PlayerState>,
    ) {
        // Update the timer for attack
        self.attack_cooldown -= dt;
        // If we have a lock, then keep pursuing the current locked player
        if let Some(player_id) = self.chasing {
            if let Some(player_pos) = player_list
                .iter()
                .find(|p| p.id == player_id)
                .map(|p| p.pos.pos())
            {
                // Condition to leave the lock: too far from player
                if player_pos.distance_to(&position.pos()) > CHASING_DISTANCE {
                    self.chasing = None;
                    self.state = MonsterStateEnum::Idle;
                } else {
                    self.go_to_target(position, player_pos, world)
                }
            } else {
                self.chasing = None;
                self.state = MonsterStateEnum::Idle;
            }
        }

        // Find a new lock
        if self.chasing.is_none() && player_list.len() > 0 {
            // Try to find any player that is in range
            if let Some(next_target) = player_list
                .iter()
                .find(|player| player.pos.distance_to(&position.pos()) < CHASING_DISTANCE)
            {
                self.chasing = Some(next_target.id);
            }
        }
    }

    fn new() -> Self {
        Self {
            state: MonsterStateEnum::Idle,
            chasing: None,
            attack_cooldown: CHASER_ATTACK_COOLDOWN,
        }
    }
}

impl Chaser {
    // Go to a target position by first rotating then going forward
    fn go_to_target(&mut self, position: &Position, target: Vector3, world: &World) {
        let forward = position.ground_direction_forward();
        let side = position.ground_direction_right();
        let direction_target = target - position.pos();
        let mut direction_target_normalize = direction_target.clone();
        direction_target_normalize.normalize();

        // Compute the dot product between the direction and the side
        let sangle = direction_target_normalize.dot(&side);

        // If the monster does not face the target, rotate
        if sangle.abs() > 0.1 {
            if sangle > 0. {
                self.state = MonsterStateEnum::TurnRight;
            } else {
                self.state = MonsterStateEnum::TurnLeft;
            }
        } else {
            let cangle = direction_target_normalize.dot(&forward);
            if cangle < 0. {
                self.state = MonsterStateEnum::TurnRight;
                return;
            }

            // Quit if close enought and try to attack
            if direction_target.norm() < ATTACK_RANGE {
                if self.attack_cooldown < 0. {
                    self.state = MonsterStateEnum::Attack;
                    self.attack_cooldown = CHASER_ATTACK_COOLDOWN;
                } else {
                    self.state = MonsterStateEnum::Idle;
                }
                return;
            }

            // If here, we have a target, we are facing it, time to move toward it !
            // Check if there is a block on the way if so jump !
            let mut pos = position.pos() + forward;
            // The block is on the ground, not facing the eyes
            pos[1] -= 1.;
            if world.cube_at(pos).is_none() {
                // Otherwise go toward the target !
                self.state = MonsterStateEnum::Forward;
            } else {
                self.state = MonsterStateEnum::Jump;
            }
        }
    }
}
