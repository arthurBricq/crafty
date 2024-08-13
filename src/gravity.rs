use std::time::Duration;

const MAX_FALL_SPEED: f32 = 2.0;
const GRAVITY: f32 = 9.81;

/// A struct to help entities deal with gravity
pub struct GravityHandler {
    is_falling: bool,
    /// Total time of the fall
    total_time: f32,
}

impl GravityHandler {
    pub fn new() -> Self {
        Self {
            is_falling: false,
            total_time: 0.
        }
    }
    
    /// Returns the current fall of the object, after the elapsed time.
    pub fn step(&mut self, is_falling: bool, elapsed: Duration) -> f32 {
        // State machine transitions
        if is_falling && !self.is_falling {
            self.is_falling = true;
        }
        
        if !is_falling && self.is_falling {
            self.is_falling = false;
            self.total_time = 0.;
            return 0.0;
        }
        
        // Compute the free-fall term and update internal logic
        if self.is_falling {
            let dt = elapsed.as_secs_f32();
            let t1 = self.total_time;
            self.total_time += dt;
            let t2 = self.total_time;
            return 0.5 * GRAVITY * (t2 * t2 - t1 * t1);
        } else {
            return  0.0;
        }
        
        
    }
}