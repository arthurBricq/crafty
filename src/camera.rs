use std::f32::consts::PI;
use std::time::Duration;

use crate::chunk::CHUNK_FLOOR;
use crate::cube::Cube;
use crate::gravity::GravityHandler;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
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
    position: Position,

    // state: MotionState
    // TODO Maybe we can build a better encapsulation of this logic
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,

    /// For handling free-fall
    gravity_handler: GravityHandler,

    /// Tuple with (Cube, touched_position)
    /// Position that the camera is currently pointing to
    /// If there is no cube, it is set to none
    touched_cube: Option<(Cube, Vector3)>,
}

impl Camera {
    /// based on right hand perspective look along the positive z-Axis
    pub fn new() -> Self {
        Self {
            position: Position::new(Vector3::new(4.0, CHUNK_FLOOR as f32 + PLAYER_HEIGHT + 5., 3.0), PI, 0.),
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            gravity_handler: GravityHandler::new(),
            touched_cube: None,
        }
    }

    pub fn step(&mut self, elapsed: Duration, world: &World) {
        // TODO The problem of hardcoding a ratio is that `dt` depends on the OpenGL performance.
        //      We need to have the physics computation done at a constant `dt`...
        //      This won't be easy in Rust.

        // Compute the next position
        let f = self.ground_direction_forward();
        let l = self.ground_direction_right();

        let mut next_pos = self.position.pos().clone();
        let mut next_pos_amplified = self.position.pos().clone();

        let amplitude = SPEED * elapsed.as_secs_f32();
        let ratio = 20.;
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
            self.position.set_position(next_pos);
        } else { 
            
        }

        self.compute_selected_cube(world);
    }

    /// Set the attribute `selected` to the cube currently being selected
    fn compute_selected_cube(&mut self, world: &World) {
        // TODO dichotomy should be much better in terms of performance
        const STEP: f32 = 0.1;
        const REACH_DISTANCE: f32 = 5.0;
        let unit_direction = self.direction();
        for i in 1..(REACH_DISTANCE / STEP) as usize {
            let query = self.position.pos() + unit_direction * i as f32 * STEP;
            // If the query position is not free, it means that we have found the selected cube
            if let Some(cube) = world.cube_at(query) {
                self.touched_cube = Some((cube.clone(), query));
                return
            }
        }
        self.touched_cube = None
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
        self.position.translate_y(5.);
    }

    pub fn down(&mut self) {
        self.position.translate_y(-1.);
    }

    pub fn debug(&mut self) {
        println!("* Camera - position   : {:?}", self.position);
        println!("*        - orientation: {:?}", self.direction());
    }

    /// Returns the normalized direction vector
    fn direction(&self) -> Vector3 {
        Vector3::new(
            self.position.yaw().cos() * self.position.pitch().cos(),
            self.position.pitch().sin(), self.position.yaw().sin() * self.position.pitch().cos())
    }

    fn ground_direction_forward(&self) -> Vector3 {
        Vector3::new(self.position.yaw().cos(), 0., self.position.yaw().sin())
    }

    fn ground_direction_right(&self) -> Vector3 {
        Vector3::new(self.position.yaw().sin(), 0., -self.position.yaw().cos())
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
        let position = self.position.pos();
        let p = [-position[0] * s[0] - position[1] * s[1] - position[2] * s[2],
            -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
            -position[0] * forward[0] - position[1] * forward[1] - position[2] * forward[2]];
        [
            [s[0], u[0], forward[0], 0.0],
            [s[1], u[1], forward[1], 0.0],
            [s[2], u[2], forward[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    pub fn mousemove(&mut self, horizontal: f32, vertical: f32, sensitivity: f32) {
        self.position.rotate_yaw(-horizontal * sensitivity);
        if vertical > 0.0 && self.position.pitch() < PI * 0.5 - 0.05 {
            self.position.rotate_pitch(vertical * sensitivity);
        } else if vertical < 0.0 && self.position.pitch() > -PI * 0.5 + 0.05 {
            self.position.rotate_pitch(vertical * sensitivity);
        }
    }

    /// Returns the optional position of the cube that the player is looking at.
    pub fn touched_cube(&self) -> Option<(Cube, Vector3)> {
        self.touched_cube
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn is_moving(&self) -> bool {
        self.d_pressed || self.a_pressed || self.w_pressed || self.s_pressed
    }
}