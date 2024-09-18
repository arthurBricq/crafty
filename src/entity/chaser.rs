use super::monster::{MonsterAction, TransitionState};
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::{server::server_state::PlayerState, world::World};

const CHASING_DISTANCE: f32 = 10.;

/// Internal state of the monster,
/// name are for convinience, they are not force to a particular action
#[derive(Clone)]
pub enum MonsterStateEnum {
    Idle,
    Forward,
    TurnLeft,
    TurnRight,
    Attack,
}

pub struct Chaser {
    state: MonsterStateEnum,
    chasing: Option<usize>,
}

impl TransitionState for Chaser {
    fn action(&self) -> MonsterAction {
        match self.state {
            MonsterStateEnum::Forward => MonsterAction::Forward,
            MonsterStateEnum::TurnLeft => MonsterAction::LeftRot,
            MonsterStateEnum::TurnRight => MonsterAction::RightRot,
            MonsterStateEnum::Attack => MonsterAction::Attack,
            _ => MonsterAction::Idle
        }
    }
    fn update(&mut self, dt: f32, position: &Position, world: &World, player_list: &Vec<PlayerState>) {
        // If we have a lock, then keep pursuing the current locked player
        if let Some(player_id) = self.chasing {
            if let Some(player_pos) = player_list.iter().find(|p| p.id == player_id).map(|p| p.pos.pos()) {
                // Condition to leave the lock: too far from player
                if player_pos.distance_to(&position.pos()) > CHASING_DISTANCE {
                    self.chasing = None
                } else {
                    self.go_to_target(position, player_pos)
                }
            } else {
                self.chasing = None
            }
        }

        // Find a new lock
        if self.chasing.is_none() && player_list.len() > 0 {
            // Try to find any player that is in range
            if let Some(next_target) = player_list.iter().find(|player| player.pos.distance_to(&position.pos()) < CHASING_DISTANCE) {
                self.chasing = Some(next_target.id);
            }
        }
    }

    fn new() -> Self {
        Self {
            state: MonsterStateEnum::Idle,
            chasing: None,
        }
    }
}


impl Chaser {
    // Go to a target position by first rotating then going forward
    fn go_to_target(&mut self, position: &Position, target: Vector3) {
        let forward = Vector3::unit_x().rotation_y(position.yaw());
        let side = Vector3::unit_z().rotation_y(position.yaw());
        let mut direction_target = target - position.pos();

        // Quit if close enought and try to attack
        if direction_target.norm() < 1. {
            self.state = MonsterStateEnum::Attack;
            return;
        }

        // Compute the dot product between the direction and the side
        direction_target.normalize();
        let sangle = direction_target.dot(&side);

        // If the monster does not face the target, rotate
        if sangle.abs() > 0.1 {
            if sangle > 0. {
                self.state = MonsterStateEnum::TurnLeft;
            } else {
                self.state = MonsterStateEnum::TurnRight;
            }
        } else {
            if sangle == 0. {
                let cangle = direction_target.dot(&forward);
                if cangle < 0. {
                    self.state = MonsterStateEnum::TurnRight;
                }
            }
            // Otherwise go toward the target !
            self.state = MonsterStateEnum::Forward;
        }
    }
}