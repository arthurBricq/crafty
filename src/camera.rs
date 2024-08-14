use std::f32::consts::PI;
use std::time::Duration;

use crate::chunk::CHUNK_FLOOR;
use crate::gravity::GravityHandler;
use crate::vector::Vector3;
use crate::world::World;

/// Travel speed [m/s] or [cube/s]
const SPEED: f32 = 2.0;

pub const PLAYER_HEIGHT: f32 = 2.;

pub enum MotionState {
    W,
    S,
    A,
    D,
    None,
}

/// First player camera
/// The state includes the position and the speed
pub struct Camera {
    /// Position of the camera
    position: Vector3,

    /// Orientation of the camera Yaw, Pitch
    rotation: [f32; 2],

    // state: MotionState
    // TODO Maybe we can build a better encapsulation of this logic
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,

    /// For handling free-fall
    gravity_handler: GravityHandler,

    /// Position that the camera is currently pointing to
    /// If there is no cube, it is set to none
    selected: Option<Vector3>,

    /// True if the next step of the camera must be for debug
    debug_next_iter: bool,
}

impl Camera {
    /// based on right hand perspective look along the positive z-Axis
    // pub fn new(collision_callback: impl FnMut([f32;3]) -> bool + 'a) -> Self {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(4.0, CHUNK_FLOOR as f32 + PLAYER_HEIGHT, 3.0),
            rotation: [PI, 0.0],
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            gravity_handler: GravityHandler::new(),
            selected: None,
            debug_next_iter: false
        }
    }

    pub fn step(&mut self, elapsed: Duration, world: &World) {
        // Compute the next position
        let f = self.ground_direction_forward();
        let l = self.ground_direction_right();
        let mut next_pos = self.position.clone();
        let mut next_pos_amplified = self.position.clone();
        let amplitude = SPEED * elapsed.as_secs_f32();
        let ratio = 10.;
        if self.w_pressed {
            next_pos += f * amplitude;
            next_pos_amplified += f * amplitude * ratio
        }
        if self.s_pressed {
            next_pos -= f * amplitude;
            next_pos_amplified -= f * amplitude * ratio
        }
        if self.d_pressed {
            next_pos += l * amplitude;
            next_pos_amplified += l * amplitude * ratio
        }
        if self.a_pressed {
            next_pos -= l * amplitude;
            next_pos_amplified -= l * amplitude * ratio
        }

        // Collision detection (xz-plane)
        let is_free = world.is_position_free(&next_pos_amplified);

        // Free-fall handling
        let is_falling = world.is_position_free_falling(&next_pos_amplified);
        let dz_fall = self.gravity_handler.step(is_falling, elapsed);
        next_pos[1] -= dz_fall;;

        // Position update
        if is_free {
            self.position = next_pos;
        }

        self.compute_selected_cube(world);
        self.debug_next_iter = false;
    }

    fn compute_selected_cube(&mut self, world: &World) {
        if self.debug_next_iter {
            println!("* Computation of selected block");
        }
        let unit_direction = self.direction();
        for i in 1..10 {
            // TODO Not sure if '* 0.5' is actually important here
            let query = self.position + unit_direction * i as f32 * 0.5;
            if self.debug_next_iter {
                println!("    - {query:?}");
            }
            // If the query position is not free, it means that we have found the selected cube
            if !world.is_position_free(&query) {
                if self.debug_next_iter {
                    println!("     SELECTED");
                }
                self.selected = Some(Vector3::new(query.x().floor(), query.y().floor(), query.z().floor()));
                return;
            }
        }
        self.selected = None
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

    pub fn jump(&mut self) {
        self.gravity_handler.jump();
    }

    pub fn up(&mut self) {
        self.position[1] += 1.;
    }

    pub fn down(&mut self) {
        self.position[1] -= 1.;
    }

    pub fn debug(&mut self) {
        self.debug_next_iter = true;
        println!("* Camera - position   : {:?}", self.position);
        println!("*        - orientation: {:?}", self.direction());
    }

    /// Returns the normalized direction vector
    fn direction(&self) -> Vector3 {
        let yaw = self.rotation[0];
        let pitch = self.rotation[1];
        Vector3::new(yaw.cos() * pitch.cos(), pitch.sin(), yaw.sin() * pitch.cos())
    }

    fn ground_direction_forward(&self) -> Vector3 {
        Vector3::new(self.rotation[0].cos(), 0., self.rotation[0].sin())
    }

    fn ground_direction_right(&self) -> Vector3 {
        Vector3::new(self.rotation[0].sin(), 0., -self.rotation[0].cos())
    }

    pub fn perspective_matrix(&self, dim: (u32, u32)) -> [[f32; 4]; 4] {
        let (width, height) = dim;
        let aspect_ratio = height as f32 / width as f32;
        let fov: f32 = std::f32::consts::PI / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;
        let f = 1.0 / (fov / 2.0).tan();
        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    /// Returns the view matrix, from the given camera parameters
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        // Compute the normalised direction vector
        let forward = self.direction();
        let camera_up = Vector3::new(0., 1., 0.);
        let mut s = camera_up.cross(&forward);
        s.normalize();
        let u = forward.cross(&s);
        let p = [-self.position[0] * s[0] - self.position[1] * s[1] - self.position[2] * s[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * forward[0] - self.position[1] * forward[1] - self.position[2] * forward[2]];
        [
            [s[0], u[0], forward[0], 0.0],
            [s[1], u[1], forward[1], 0.0],
            [s[2], u[2], forward[2], 0.0],
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

    /// Returns the optional position of the cube that the player is looking at.
    pub fn selected(&self) -> Option<Vector3> {
        self.selected
    }

}