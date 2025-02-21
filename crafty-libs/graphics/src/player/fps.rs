use std::time::Duration;
use bounded_vec_deque::BoundedVecDeque;

const TIME_TO_PRINT: f32 = 1.5;
const BUFFER_LEN: usize = 10;

/// A struct in charge of computing the FPS and printing it to the console.
pub struct FpsManager {
    counter: f32,
    buffer: BoundedVecDeque<f32>,
    fps: f32,
}

impl FpsManager {
    pub fn new() -> Self {
        Self {
            counter: 0.,
            fps: 0.,
            buffer: BoundedVecDeque::new(BUFFER_LEN),
        }
    }

    pub fn step(&mut self, elapsed: Duration) {
        let dt = elapsed.as_secs_f32();
        self.buffer.push_front(dt);
        self.counter += dt;
        if self.counter > TIME_TO_PRINT {
            self.counter = 0.0;
            self.fps = self.buffer.len() as f32 / self.buffer.iter().sum::<f32>();
            self.print_statistics()
        }
    }

    fn print_statistics(&self) {
        println!("fps = {}", self.fps);
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}
