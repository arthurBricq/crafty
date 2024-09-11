use crate::primitives::vector::Vector3;
use crate::{server::server_state::PlayerState, world::World};
use crate::primitives::position::{self, Position};
use super::monster::{MonsterAction, TransitionState};

const CHASING_DISTANCE: f32 = 10.;

/// Internal state of the monster,
/// name are for convinience, they are not force to a particular action
#[derive(Clone)]
pub enum MonsterStateEnum {
    Idle,
    Forward,
    TurnLeft,
    TurnRight,
    Attack
}

pub struct Chaser {
    state: MonsterStateEnum,
    target: Vector3
}

impl TransitionState for Chaser {
    fn new() -> Self {
        Self {
            state: MonsterStateEnum::Idle,
            target: Vector3::empty()
        }
    }
    fn action(&self) -> MonsterAction {
        match self.state {
            MonsterStateEnum::Forward => MonsterAction::Forward,
            MonsterStateEnum::TurnLeft => MonsterAction::LeftRot,
            MonsterStateEnum::TurnRight => MonsterAction::RightRot,
            MonsterStateEnum::Attack => MonsterAction::Attack,
            _ => MonsterAction::Idle
            
        }
    }

    fn update(&mut self, dt: f32, position: &Position, world: &World, player_list: Vec<PlayerState>) {
        if player_list.len() > 0 {
            // Try to find the closest player
            let mut index_closest: Option<usize> = Option::None;
            let mut closest_norm = 10.;
            let mut norm = 0.;
            for i in 0..player_list.len() {
                norm = (player_list[i].pos.pos() - position.pos()).norm();
                if  norm < CHASING_DISTANCE && norm < closest_norm {
                    closest_norm = norm;
                    index_closest = Some(i);
                }
            }
            match index_closest {
                    // A player is close enought, target it
                Some(index) => {
                    self.target = player_list[index].pos.pos();
                    self.go_to_target(position)
                },
                // No player is close enought
                _ => self.state = MonsterStateEnum::Idle
            }
        }
    }
}


impl Chaser {
    // Go to a target position by first rotating then going forward
    fn go_to_target(&mut self, position: &Position) {

        let forward = Vector3::unit_x().rotation_y(position.yaw());
        // Vector on the side of the player
        let side = Vector3::unit_z().rotation_y(position.yaw());
        let mut direction_target = self.target - position.pos();
        // Quit if close enought and try to attack
        if direction_target.norm() < 1. {
            self.state = MonsterStateEnum::Attack;
            return;
        }
        direction_target.normalize();
        // Compute the dot product between the direction and the side
        let sangle = direction_target.dot(&side);

        // If the monster does not face the targer, rotate
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