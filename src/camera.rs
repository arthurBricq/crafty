use std::f32::consts::PI;
use std::time::Duration;
use crate::chunk::CHUNK_FLOOR;
use crate::world::World;

const SPEED_INC: f32 = 0.5;
const MAX_SPEED: f32 = 2.0;
const MIN_SPEED: f32 = 0.1;

pub enum MotionState {
    W,S,A,D,None
}

/// First player camera
/// The state includes the position and the speed
pub struct Camera<'a> {
    position: [f32; 3],
    speed: [f32; 3],
    /// Yaw, Pitch
    rotation: [f32; 2],
    
    // state: MotionState
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,
    
    // Collision callcack
    // collision_callback: Box<dyn FnMut([f32;3]) -> bool + 'a>
    world: &'a World
}

impl<'a> Camera<'a> {
    /// based on right hand perspective look along the positive z-Axis
    // pub fn new(collision_callback: impl FnMut([f32;3]) -> bool + 'a) -> Self {
    pub fn new(world: &'a World) -> Self {
        Self {
            position: [10.0, CHUNK_FLOOR as f32 + 2., 3.0],
            speed: [0.; 3],
            rotation: [PI, 0.0],
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            world
        }
    }

    pub fn step(&mut self, elapsed: Duration) {
        println!("speed = {:?}, dt = {:?}", self.speed, elapsed.as_secs_f32());
        println!("{} {} {} {}", self.w_pressed, self.s_pressed, self.a_pressed, self.d_pressed);
        // TODO I have to improve this function.
        //      My idea so far is to say "speed = direction vector"
        //      w pressed => speed += direction vector
        //      d pressed => speed -= direction vector
        //      etc...
        //      But it is definitely not so easy, how to implement the deceleration ?
        
        // Update the speed vector
        if self.w_pressed {
            self.speed[0] += SPEED_INC * self.rotation[0].cos() * self.rotation[1].cos();
            // self.speed[1] += SPEED_INC * self.rotation[1].sin();
            self.speed[2] += SPEED_INC * self.rotation[0].sin() * self.rotation[1].cos();
            self.clamp_speed();
        }
        if self.s_pressed {
            self.speed[0] -= SPEED_INC * self.rotation[0].cos() * self.rotation[1].cos();
            // self.speed[1] -= SPEED_INC * self.rotation[1].sin();
            self.speed[2] -= SPEED_INC * self.rotation[0].sin() * self.rotation[1].cos();
            self.clamp_speed();
        }
        if self.d_pressed {
            self.speed[0] += SPEED_INC * self.rotation[0].sin();
            self.speed[2] -= SPEED_INC * self.rotation[0].cos();
            self.clamp_speed();
        }
        if self.a_pressed {
            self.speed[0] -= SPEED_INC * self.rotation[0].sin();
            self.speed[2] += SPEED_INC * self.rotation[0].cos();
            self.clamp_speed();
        }
        
        // If no key is being pressed, reduced the speed
        if !self.w_pressed && !self.s_pressed && !self.a_pressed && !self.d_pressed {
            if self.speed[0].abs() > MIN_SPEED {
                self.speed[0] /= 2.;
            } else {
                self.speed[0] = 0.;
            }
            if self.speed[1].abs() > MIN_SPEED {
                self.speed[1] /= 2.;
            } else {
                self.speed[1] = 0.;
            }
            if self.speed[2].abs() > MIN_SPEED {
                self.speed[2] /= 2.;
            } else {
                self.speed[2] = 0.;
            }
        }
        
        // Compute the new position
        let dx = elapsed.as_secs_f32() * self.speed[0];
        let dy = elapsed.as_secs_f32() * self.speed[1];
        let dz = elapsed.as_secs_f32() * self.speed[2];
        let new_pos = [self.position[0] + dx, self.position[1] + dy, self.position[2] + dz];
        
        if self.world.is_position_free(&new_pos) {
            // Update the position if the world is free
            self.position[0] += dx;
            self.position[1] += dy;
            self.position[2] += dz;
        }
        
    }

    fn clamp_speed(&mut self) {
        self.speed[0] = self.speed[0].clamp(-MAX_SPEED, MAX_SPEED);
        self.speed[1] = self.speed[1].clamp(-MAX_SPEED, MAX_SPEED);
        self.speed[2] = self.speed[2].clamp(-MAX_SPEED, MAX_SPEED);
    }

    pub fn toggle_state(&mut self, state: MotionState) {
        match state {
            MotionState::W => self.w_pressed = !self.w_pressed,
            MotionState::S => self.s_pressed = !self.s_pressed,
            MotionState::A => self.a_pressed = !self.a_pressed,
            MotionState::D => self.d_pressed = !self.d_pressed,
            MotionState::None => {}
        }
    }

    pub fn up(&mut self) {
        self.position[1] += 1.;
    }

    pub fn down(&mut self) {
        self.position[1] -= 1.;
    }

    /// Returns the view matrix, from the given camera parameters
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        // Compute the normalised direction vector
        let f = {
            let yaw = self.rotation[0];
            let pitch = self.rotation[1];
            let dir: [f32; 3] = [yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos()];
            dir
        };

        let camera_up: [f32; 3] = [0., 1., 0.];

        let s = [camera_up[1] * f[2] - camera_up[2] * f[1],
            camera_up[2] * f[0] - camera_up[0] * f[2],
            camera_up[0] * f[1] - camera_up[1] * f[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0]];

        let p = [-self.position[0] * s_norm[0] - self.position[1] * s_norm[1] - self.position[2] * s_norm[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2]];

        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    pub fn mousemove(&mut self, horizontal: f32, vertical: f32, sensitivity: f32) {
        self.rotation[0] -= horizontal * sensitivity;

        // don't let the player turn upside down
        if vertical > 0.0 && self.rotation[1] < PI * 0.5 {
            self.rotation[1] += vertical * sensitivity;
        } else if vertical < 0.0 && self.rotation[1] > -PI * 0.5 {
            self.rotation[1] += vertical * sensitivity;
        }
    }
}