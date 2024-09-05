use std::f32::consts::PI;
use std::time::Duration;

use crate::chunk::CHUNK_FLOOR;
use crate::vector::Vector3;
use crate::world::World;
use crate::aabb::{AABB};

/// Travel speed [m/s] or [cube/s]
const SPEED: f32 = 4.;
// TODO for some obscure reason, actual speed is lower than that. Perhaps the dt
// is wrong, or yet again the collision ?

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
    velocity: Vector3,

    /// Orientation of the camera Yaw, Pitch
    rotation: [f32; 2],

    // state: MotionState
    // TODO Maybe we can build a better encapsulation of this logic
    w_pressed: bool,
    s_pressed: bool,
    a_pressed: bool,
    d_pressed: bool,

    /// Position that the camera is currently pointing to
    /// If there is no cube, it is set to none
    touched_cube: Option<Vector3>,

    in_air: bool,
    jump_time: f32, // > 0 if we have to apply jump force
}

impl Camera {
    /// based on right hand perspective look along the positive z-Axis
    pub fn new() -> Self {
        Self {
            position: Vector3::new(4.0, 24. + CHUNK_FLOOR as f32, 3.0),
	    velocity: Vector3::new(0., 0., 0.),
            rotation: [PI, 0.0],
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            touched_cube: None,
	    in_air: true, // will be updated every frame anyway
	    jump_time: 0.,
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
        let f = self.ground_direction_forward();
        let l = self.ground_direction_right();
        let mut next_pos = self.position.clone();
	let mut dt = elapsed.as_secs_f32();

	// add gravity
	if self.in_air {
	    self.velocity += Vector3::new(0., -9.81, 0.) * dt;
	}

	// add jump force
	if self.jump_time > 0. {
	    self.velocity += Vector3::new(0., 4. * 9.81, 0.) * dt;
	    self.jump_time -= dt;
	}
	
	// TODO will have to do something cleaner when other sources of
	// horizontal velocity will be implemented
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
	// movements. However, there are no slopes in the cubes for now, so it
	// is not necessary
	
	// update in_air
	let displacement = Vector3::new(0., -1e-5, 0.);
	self.in_air = ! world.collides(&Self::make_aabb(&(self.position + displacement)));

        self.compute_selected_cube(world);
    }

    /// Integrate the velocity to move the camera, with collision. Returns the
    /// dt (in seconds), which can be smaller than `dt` if there is a collision.
    fn move_with_collision(&mut self, dt: f32, world: &World) -> f32 {
	let target = Self::make_aabb(&(self.position + self.velocity * dt));
	
	let (collision_time, normal) =
	    world.collision_time(&Self::make_aabb(&self.position), &target, &self.velocity);
	if collision_time >= dt {
	    // can move straight away
	    self.position += self.velocity * dt;

	    dt
	}
	else {
	    // we want to put a margin, to avoid collision even with floats rounding
	    
	    // TODO if we are here, we can assume the velocity is nonzero I
	    // think, but I am not sure
	    let dtmargin = 1e-5 / self.velocity.norm();
	    self.position += self.velocity * (collision_time - dtmargin);

	    // remove component of velocity along the normal
	    let vnormal = normal * normal.dot(&self.velocity);
	    self.velocity = self.velocity - vnormal;

	    collision_time
	}
    }
    
    fn make_aabb(position: &Vector3) -> AABB {
	let diameter = 0.5;
	let height = 1.8;
	let forehead = 0.1;
	
	AABB {
	    north: position.z() + diameter / 2.,
	    south: position.z() - diameter / 2.,
	    top: position.y() + forehead,
	    bottom: position.y() + forehead - height,
	    east: position.x() + diameter / 2.,
	    west: position.x() - diameter / 2.,
	}
    }
    
    /// Set the attribute `selected` to the cube currently being selected
    fn compute_selected_cube(&mut self, world: &World) {
        // TODO dichotomy should be much better in terms of performance
        const STEP: f32 = 0.1;
        const REACH_DISTANCE: f32 = 5.0;
        let unit_direction = self.direction();
        for i in 1..(REACH_DISTANCE / STEP) as usize {
            let query = self.position + unit_direction * i as f32 * STEP;
            // If the query position is not free, it means that we have found
            // the selected cube
            if !world.is_position_free(&query) {
                self.touched_cube = Some(query);
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
	    self.jump_time = 0.1;
	}
    }

    pub fn up(&mut self) {
        self.position[1] += 5.;
    }

    pub fn down(&mut self) {
        self.position[1] -= 1.;
    }

    pub fn debug(&mut self) {
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
        if vertical > 0.0 && self.rotation[1] < PI * 0.5 - 0.05 {
            self.rotation[1] += vertical * sensitivity;
        } else if vertical < 0.0 && self.rotation[1] > -PI * 0.5 + 0.05 {
            self.rotation[1] += vertical * sensitivity;
        }
    }

    /// Returns the optional position of the cube that the player is looking at.
    pub fn touched_cube(&self) -> Option<Vector3> {
        self.touched_cube
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }
}
