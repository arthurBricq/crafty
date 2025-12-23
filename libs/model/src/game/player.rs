use crate::entity::humanoid::humanoid_aabb;
use primitives::position::Position;
use primitives::vector::Vector3;
use std::f32::consts::PI;
use std::time::Duration;
use crate::collision::collidable::{Collidable, CollisionData};
use crate::game::input::{MotionState, PlayerInputStatus};
use crate::world::cube::Cube;
use crate::world::world::World;

pub const CLICK_TIME_TO_BREAK: f32 = 2.0;

/// Travel speed [m/s] or [cube/s]
const SPEED: f32 = 4.0;
// TODO for some obscure reason, actual speed is lower than that. Perhaps the dt
// is wrong, or yet again the collision ?

/// Velocity [cube/s] added when jumping
pub const JUMP_VELOCITY: f32 = 7.;

// TODO same problem
pub const GRAVITY_ACCELERATION_VECTOR: Vector3 = Vector3::new(0., -2. * 9.81, 0.);

pub const PLAYER_MARGIN: f32 = 1e-5;

pub const PLAYER_HEIGHT: f32 = 1.8;
pub const DIAMETER: f32 = 0.5;
pub const FOREHEAD: f32 = 0.1;

/// Represents the physical state of a player, on the client side.
/// This means:
/// - position
/// - orientation
/// - speed
///
/// It handles
/// - Collision detection
/// - Cube selection
pub struct Player {
    /// Position of the camera
    position: Position,

    /// Speed of the player
    velocity: Vector3,

    /// Currently logical inputs pressed
    input_status: PlayerInputStatus,

    /// Tuple with (Cube, touched_position)
    /// Position that the camera is currently pointing to
    /// If there is no cube, it is set to none
    touched_cube: Option<Cube>,

    in_air: bool,
}

impl Player {
    /// based on right hand perspective look along the positive z-Axis
    pub fn new() -> Self {
        Self {
            position: Position::empty(),
            velocity: Vector3::new(0., 0., 0.),
            input_status: PlayerInputStatus::new(),
            touched_cube: None,
            in_air: true, // will be updated every frame anyway
        }
    }

    pub fn step(&mut self, elapsed: Duration, world: &World) {
        // Compute the next position
        let dt = elapsed.as_secs_f32();

        // add gravity
        if self.in_air {
            self.velocity += GRAVITY_ACCELERATION_VECTOR * dt;
        }

        {
            let controls_vel = self.controls_velocity();
            self.velocity[0] = controls_vel[0];
            self.velocity[2] = controls_vel[2];
        }

        if self.input_status.jump() {
            self.jump();
        }

        let mut dt = elapsed.as_secs_f32();
        loop {
            dt = dt - self.move_with_collision(dt, world);
            if dt <= 0. {
                break;
            }
        }

        // update in_air
        let displacement = Vector3::new(0., -2.0 * PLAYER_MARGIN, 0.);
        self.in_air = !world.collides(&humanoid_aabb(&(&self.position + displacement)));
        self.compute_selected_cube(world);
    }

    pub fn toggle_state(&mut self, element: MotionState, pressed: bool) {
        self.input_status.set_input(element, pressed);
    }

    /// Sets the position of the player to the given one, without collision checks
    pub fn set_position(&mut self, position: Position) {
        self.position = position
    }

    pub fn left_click(&self) -> bool {
        self.input_status.left_click()
    }

    pub fn left_click_time(&self) -> f32 {
        self.input_status.click_time()
    }

    pub fn reset_click_time(&mut self) {
        self.input_status.reset_click_time()
    }

    pub fn add_click_time(&mut self, click_time: f32) {
        self.input_status.add_click_time(click_time)
    }

    pub fn jump(&mut self) {
        if !self.in_air {
            self.velocity[1] = JUMP_VELOCITY;
        }
    }

    pub fn up(&mut self) {
        self.position.translate_y(5.);
    }

    pub fn down(&mut self) {
        self.position.translate_y(-1.);
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
        let p = [
            -position[0] * s[0] - position[1] * s[1] - position[2] * s[2],
            -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
            -position[0] * forward[0] - position[1] * forward[1] - position[2] * forward[2],
        ];
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
    pub fn selected_cube(&self) -> Option<Cube> {
        self.touched_cube
    }

    pub fn is_selecting_cube(&self) -> bool {
        self.touched_cube.is_some()
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Check if tha player is colliding with a block position
    pub fn is_in(&self, cube_pos: Vector3) -> bool {
        let cube_aabb = Cube::cube_aabb(cube_pos);
        let player_aabb = &humanoid_aabb(&self.position);

        return cube_aabb.collides(player_aabb);
    }

    pub fn debug(&mut self) {
        tracing::debug!("* Camera - position   : {:?}", self.position);
        tracing::debug!("*        - orientation: {:?}", self.direction());
    }

    /// Returns the velocity vector due to the controls purely (ignoring
    /// collisions or gravity)
    fn controls_velocity(&self) -> Vector3 {
        let f = self.position.ground_direction_forward();
        let l = self.position.ground_direction_right();

        let mut displacement = Vector3::empty();
        if self.input_status.forward() {
            displacement += f * SPEED;
        }
        if self.input_status.backward() {
            displacement -= f * SPEED;
        }
        if self.input_status.right() {
            displacement += l * SPEED;
        }
        if self.input_status.left() {
            displacement -= l * SPEED;
        }

        displacement
    }

    /// Set the attribute `selected` to the cube currently being selected
    fn compute_selected_cube(&mut self, world: &World) {
        let position = self.position.pos();
        let direction = self.direction();

        // How to find which cube is selected by the camera ?
        // 1. loop through all the visible cubes near the player
        // 2. compute the intersection between the player's ray and the cube
        // 3. keep the intersection with the shortest distance

        let mut current_best: Option<(f32, Cube)> = None;
        for cube in world
            .cubes_near_player(position)
            .filter_map(|c| *c)
            .filter(|c| c.is_visible())
            .filter(|c| c.position().distance_to(&position) < 6.)
        {
            if let Some(result) = cube.intersection_with(position, direction) {
                if current_best.is_none() || result < current_best.unwrap().0 {
                    current_best = Some((result, cube.clone()));
                }
            }
        }

        self.touched_cube = current_best.map(|(_, cube)| cube)
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

    /// Returns the normalized direction vector
    pub fn direction(&self) -> Vector3 {
        Vector3::new(
            self.position.yaw().cos() * self.position.pitch().cos(),
            self.position.pitch().sin(),
            self.position.yaw().sin() * self.position.pitch().cos(),
        )
    }

    /// Returns true if the player is asking to break a cube
    pub fn is_time_to_break_over(&mut self, dt: f32) -> bool {
        if self.is_selecting_cube() && self.left_click() {
            self.add_click_time(dt);
            if self.left_click_time() >= CLICK_TIME_TO_BREAK {
                self.reset_click_time();
                return true;
            }
        }
        false
    }
}
