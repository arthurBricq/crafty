use std::ops::{Add, AddAssign};
use std::str::from_utf8;
use crate::chunk::CHUNK_FLOOR;
use crate::primitives::vector::Vector3;

/// Position of an entity in a 3D world
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pos: Vector3,
    yaw: f32,
    pitch: f32,
}

impl Position {
    pub fn empty() -> Self {
        Self {
            pos: Vector3::empty(),
            yaw: 0.,
            pitch: 0.
        }
    }
    
    pub fn spawn_position() -> Self {
        Self {
            pos: Vector3::new(0., CHUNK_FLOOR as f32 + 3., 0.),
            yaw: 0.,
            pitch: 0.
        }
    }
    
    pub fn new(pos: Vector3, yaw: f32, pitch: f32) -> Self {
        Self { pos, yaw, pitch }
    }

    pub fn new_vec(x: f32, y: f32, z: f32) -> Self {
        Self { pos: Vector3::new(x, y, z), yaw: 0., pitch: 0. }
    }

    pub fn from_pos(pos: Vector3) -> Self {
        Self { pos, yaw: 0., pitch: 0. }
    }
    
    /// Send the player in the air
    pub fn raise(&mut self) {
        self.pos[1] += 2. * CHUNK_FLOOR as f32;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        format!("{},{},{},{},{}", self.pos.x(), self.pos.y(), self.pos.z(), self.yaw, self.pitch).into_bytes()
    }

    pub fn from_bytes(bytes_to_parse: &[u8]) -> Self {
        let text = from_utf8(bytes_to_parse).unwrap();
        let mut iter = text.split(',');
        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        let z = iter.next().unwrap().parse::<f32>().unwrap();
        let yaw = iter.next().unwrap().parse::<f32>().unwrap();
        let pitch = iter.next().unwrap().parse::<f32>().unwrap();
        Self { pos: Vector3::new(x, y, z), yaw, pitch }
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn pos(&self) -> Vector3 {
        self.pos
    }

    pub fn x(&self) -> f32 {
        self.pos.x()
    }

    pub fn y(&self) -> f32 {
        self.pos.y()
    }

    pub fn z(&self) -> f32 {
        self.pos.z()
    }

    pub fn set_position(&mut self, pos: Vector3) {
        self.pos = pos;
    }

    pub fn rotate_yaw(&mut self, inc: f32) {
        self.yaw += inc;
    }

    pub fn rotate_pitch(&mut self, inc: f32) {
        self.pitch += inc;
    }

    pub fn translate_y(&mut self, inc: f32) {
        self.pos[1] += inc
    }
}

impl AddAssign<Vector3> for Position {
    fn add_assign(&mut self, rhs: Vector3) {
        self.pos += rhs
    }
}

impl Add<Vector3> for &Position {
    type Output = Position;

    fn add(self, rhs: Vector3) -> Self::Output {
        Position {
            pos: self.pos + rhs,
            yaw: self.yaw,
            pitch: self.pitch
        }
    }
}
