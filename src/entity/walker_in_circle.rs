use crate::world::World;
use crate::primitives::position::Position;
use super::monster::{MonsterAction, TransitionState};

/// Internal state of the monster,
/// name are for convinience, they are not force to a particular action
#[derive(Clone)]
pub enum MonsterStateEnum {
    Idle,
    Forward,
    TurnLeft,
}

pub struct WalkInCercle {
    state: MonsterStateEnum,
    timer: f32,
}

impl TransitionState for WalkInCercle {
    fn new() -> Self {
        Self {
            state: MonsterStateEnum::Idle,
            timer: 0.
        }
    }
    fn action(&self) -> MonsterAction {
        match self.state {
            MonsterStateEnum::Forward => MonsterAction::Forward,
            MonsterStateEnum::TurnLeft => MonsterAction::LeftRot,
            _ => MonsterAction::Idle
            
        }
    }

    fn update(&mut self, dt: f32, position: &Position, world: &World) {
        if self.timer - dt < 0. {
            match self.state {
                MonsterStateEnum::Idle => {
                    self.state = MonsterStateEnum::Forward;
                    self.timer = 3.;
                }
                MonsterStateEnum::Forward => {
                    self.state = MonsterStateEnum::TurnLeft;
                    self.timer = 1.;
                }
                MonsterStateEnum::TurnLeft => {
                    self.state = MonsterStateEnum::Idle;
                    self.timer = 2.;
                }
                _ => ()
                
            }
        } else {
            self.timer -= dt;
        }
    }
}