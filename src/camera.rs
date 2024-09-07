use crate::aabb::AABB;
use crate::chunk::CHUNK_FLOOR;
use crate::collidable::{Collidable, CollisionData};
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::world::World;
use std::f32::consts::PI;
use std::time::Duration;
use crate::cube::Cube;

/// Travel speed [m/s] or [cube/s]
const SPEED: f32 = 6.;
// TODO for some obscure reason, actual speed is lower than that. Perhaps the dt
// is wrong, or yet again the collision ?

/// Velocity [cube/s] added when jumping
const JUMP_VELOCITY: f32 = 7.;

// TODO same problem
const GRAVITY_ACCELERATION_VECTOR: Vector3 = Vector3::new(0., -2. * 9.81, 0.);

pub const PLAYER_HEIGHT: f32 = 1.8;

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

    /// Speed of the player
    velocity: Vector3,

    // TODO Maybe we can build a better encapsulation of this logic
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,

    /// Tuple with (Cube, touched_position)
    /// Position that the camera is currently pointing to
    /// If there is no cube, it is set to none
    touched_cube: Option<(Cube, Vector3)>,

    in_air: bool,
}


impl Camera {
    /// based on right hand perspective look along the positive z-Axis
    pub fn new() -> Self {
        Self {
            position: Position::new(Vector3::new(4.0, CHUNK_FLOOR as f32 + 24., 3.0), PI, 0.),
            velocity: Vector3::new(0., 0., 0.),
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            touched_cube: None,
            in_air: true, // will be updated every frame anyway
        }
    }

    /// Returns the velocity vector due to the controls purely (ignoring
    /// collisions or gravity)
    fn controls_velocity(&self) -> Vector3 {
        let f = self.ground_direction_forward();
        let l = self.ground_direction_right();

        let mut displacement = Vector3::empty();
        if self.w_pressed {
            displacement += f * SPEED;
        }
        if self.s_pressed {
            displacement -= f * SPEED;
        }
        if self.d_pressed {
            displacement += l * SPEED;
        }
        if self.a_pressed {
            displacement -= l * SPEED;
        }

        displacement
    }

    pub fn step(&mut self, elapsed: Duration, world: &World) {
        // Compute the next position
        let dt = elapsed.as_secs_f32();

        // add gravity
        if self.in_air {
            self.velocity += GRAVITY_ACCELERATION_VECTOR * dt;
        }

        // TODO will have to do something cleaner when other sources of
        //      horizontal velocity will be implemented
        {
            let controls_vel = self.controls_velocity();
            self.velocity[0] = controls_vel[0];
            self.velocity[2] = controls_vel[2];
        }

        let mut dt = elapsed.as_secs_f32();
        dt = dt - self.move_with_collision(dt, world);
        if dt > 0. {
            dt = dt - self.move_with_collision(dt, world);
        }
        // TODO we may want to do it a third time, to handle x-y-z diagonal
        //      movements. However, there are no slopes in the cubes for now, so it
        //      is not necessary

        // update in_air
        let displacement = Vector3::new(0., -1e-5, 0.);
        self.in_air = !world.collides(&Self::make_aabb(&(&self.position + displacement)));

        self.compute_selected_cube(world);
    }

    /// Integrate the velocity to move the camera, with collision. Returns the
    /// dt (in seconds), which can be smaller than `dt` if there is a collision.
    fn move_with_collision(&mut self, dt: f32, world: &World) -> f32 {
        let target = Self::make_aabb(&(&self.position + self.velocity * dt));

        let collision =
            world.collision_time(&Self::make_aabb(&self.position), &target, &self.velocity)
            .unwrap_or(CollisionData { time: f32::MAX, normal: Vector3::empty() });

        if collision.time >= dt {
            // can move straight away
            self.position += self.velocity * dt;

            dt
        } else {
            // we want to put a margin, to avoid collision even with floats rounding

            // TODO if we are here, we can assume the velocity is nonzero I
            // think, but I am not sure
            let dtmargin = 1e-5 / self.velocity.norm();
            self.position += self.velocity * (collision.time - dtmargin);

            // remove component of velocity along the normal
            let vnormal = collision.normal * collision.normal.dot(&self.velocity);
            self.velocity = self.velocity - vnormal;

            collision.time
        }
    }

    fn make_aabb(position: &Position) -> AABB {
        const DIAMETER: f32 = 0.5;
        const FOREHEAD: f32 = 0.1;

        AABB::new(
            position.z() + DIAMETER / 2.,
            position.z() - DIAMETER / 2.,
            position.y() + FOREHEAD,
            position.y() - PLAYER_HEIGHT + FOREHEAD,
            position.x() + DIAMETER / 2.,
            position.x() - DIAMETER / 2.
        ).unwrap()
    }

    /// Set the attribute `selected` to the cube currently being selected
    fn compute_selected_cube(&mut self, world: &World) {
        // TODO dichotomy should be much better in terms of performance
        const STEP: f32 = 0.1;
        const REACH_DISTANCE: f32 = 5.0;
        let unit_direction = self.direction();
        for i in 1..(REACH_DISTANCE / STEP) as usize {
            // If the query position is not free, it means that we have found
            // the selected cube
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

    /// Returns the optional (cube, position on the cube) of the cube that the player is looking at.
    pub fn selection_internals(&self) -> Option<(Cube, Vector3)> {
        self.touched_cube
    }
    
    pub fn is_selecting_cube(&self) -> bool {
        self.touched_cube.is_some()
    }

    /// Returns the optional position of the cube that the player is looking at.
    pub fn selected_cube(&self) -> Option<Cube> {
        self.touched_cube.map(|(cube, _)| cube)
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn is_moving(&self) -> bool {
        self.d_pressed || self.a_pressed || self.w_pressed || self.s_pressed
    }
}
