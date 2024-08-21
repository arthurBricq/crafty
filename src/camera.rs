use std::f32::consts::PI;
use std::time::Duration;

use crate::chunk::CHUNK_FLOOR;
use crate::gravity::GravityHandler;
use crate::vector::Vector3;
use crate::world::World;
use crate::aabb::{AABB, DisplacementStatus};

/// Travel speed [m/s] or [cube/s]
const SPEED: f32 = 0.20;

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
    velocity: Vector3,

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
    touched_cube: Option<Vector3>,
}

impl Camera {
    /// based on right hand perspective look along the positive z-Axis
    pub fn new() -> Self {
        Self {
            position: Vector3::new(4.0, CHUNK_FLOOR as f32 + PLAYER_HEIGHT, 3.0),
	    velocity: Vector3::new(0., 0., 0.),
            rotation: [PI, 0.0],
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            gravity_handler: GravityHandler::new(),
            touched_cube: None,
        }
    }

    /// Returns the velocity vector due to the controls purely (ignoring
    /// collisions or gravity)
    fn controls_velocity(&self) -> Vector3 {
	let f = self.ground_direction_forward();
        let l = self.ground_direction_right();
	
	let amplitude = SPEED;
	let mut displacement = Vector3::empty();
        if self.w_pressed {
            displacement += f * amplitude;
        }
        if self.s_pressed {
            displacement -= f * amplitude;
        }
        if self.d_pressed {
            displacement += l * amplitude;
        }
        if self.a_pressed {
            displacement -= l * amplitude;
        }

	displacement
    }
    
    pub fn step(&mut self, elapsed: Duration, world: &World) {
        // Compute the next position
        let f = self.ground_direction_forward();
        let l = self.ground_direction_right();
        let mut next_pos = self.position.clone();

	// idea:
	// - compute all applied acceleration
	// - integrate to get speed
	// - floor the speed with the movement vector if needed
	// - also take into account drag by capping y speed
	// - integrate to get position
	// - if collision:
	//   - find the the best position without collision
	//   - reset speed normal to collision

	dbg!("start of round");
	let mut acceleration = Vector3::empty();
	acceleration += Vector3::new(0., -9., 0.); // gravity
	dbg!("velocity before");
	dbg!(self.velocity);

	// integrate to get velocity
	self.velocity += acceleration * elapsed.as_secs_f32();

	dbg!("velocity after accel");
	dbg!(self.velocity);
	
	let control_vel = self.controls_velocity();
	// clamp the velocity to the control one
	for n in [0, 2] { // do this in plane only
	    let speed = control_vel[n];
	    if speed > 0. {
		self.velocity[n] = self.velocity[n].max(speed)
	    } else if speed < 0. {
		self.velocity[n] = self.velocity[n].min(speed)
	    } else if speed == 0. {
		self.velocity[n] = 0.
	    }
	}

	dbg!("velocity after clamping");
	dbg!(self.velocity);
	
	// integrate to get position
	let mut displacement = self.velocity * elapsed.as_secs_f32();


	// idea:

	// use signed distances (negative for collision) / vectors
	
	// knowing the displacement, the collision method will now from which
	// direction we are coming, and hence what is positive or negative

	// they don't even need to know the displacement, just a 3 state for
	// each axis: forward, backward, and still

	// for each axis, just take the min among all the collisions
	let diameter = 0.5;
	let height = 1.8;
	let forehead = 0.1;

	let displacement_status = DisplacementStatus::from_vector(displacement);
	
	let signature = world.collision(&AABB {
	    north: self.position.z() + diameter / 2.,
	    south: self.position.z() - diameter / 2.,
	    top: self.position.y() + forehead,
	    bottom: self.position.y() + forehead - height,
	    east: self.position.x() + diameter / 2.,
	    west: self.position.x() - diameter / 2.,
	}, &displacement_status);

	dbg!(self.position);
	dbg!(signature);
	dbg!(&displacement);
	dbg!(&displacement_status);

	// deal with collision
	for n in 0..2 {
	    if signature[n] <= 0. {
		displacement[n] += displacement_status[n].to_scalar() * signature[n];
		// reset velocity along this axis
		self.velocity[n] = 0.;
	    }
	}

	dbg!(&displacement);

	self.position += displacement;
	
	// next_pos += self.movement_vector(elapsed);
	// gravity
	// next_pos += 
	

        // // Collision detection (xz-plane)
        // let is_free = world.is_position_free(&next_pos_amplified);

        // // Free-fall handling
        // let is_falling = world.is_position_free_falling(&next_pos_amplified);
        // let dz_fall = self.gravity_handler.step(is_falling, elapsed);
        // next_pos[1] -= dz_fall;;

        // // Position update
        // if is_free {
        //     self.position = next_pos;
        // } else { 
            
        // }

        self.compute_selected_cube(world);
    }

    /// Set the attribute `selected` to the cube currently being selected
    fn compute_selected_cube(&mut self, world: &World) {
        // TODO dichotomy should be much better in terms of performance
        const STEP: f32 = 0.1;
        const REACH_DISTANCE: f32 = 5.0;
        let unit_direction = self.direction();
        for i in 1..(REACH_DISTANCE / STEP) as usize {
            let query = self.position + unit_direction * i as f32 * STEP;
            // If the query position is not free, it means that we have found the selected cube
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
        self.gravity_handler.jump();
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
