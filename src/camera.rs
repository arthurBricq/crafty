use std::f32::consts::PI;
use crate::chunk::CHUNK_FLOOR;

pub struct Camera {
    position: [f32; 3],
    /// Yaw, Pitch
    rotation: [f32; 2],
}

impl Camera {
    /// based on right hand perspective look along the positive z-Axis
    pub fn new() -> Self {
        Self {
            position: [10.0, CHUNK_FLOOR as f32 + 2., 3.0],
            rotation: [PI, 0.0],
        }
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

        // TODO this one needs to change
        //      actually maybe you don't need to change it...
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

    pub fn forward(&mut self, speed: f32) {
        self.position[0] += speed * self.rotation[0].cos() * self.rotation[1].cos();
        self.position[1] += speed * self.rotation[1].sin();
        self.position[2] += speed * self.rotation[0].sin() * self.rotation[1].cos();
    }

    pub fn orthogonal(&mut self, speed: f32) {
        self.position[0] += speed * self.rotation[0].sin();
        self.position[2] -= speed * self.rotation[0].cos();
    }

    pub fn up(&mut self, speed: f32) {
        self.position[1] += speed;
    }

    pub fn mousemove(&mut self, horizontal: f32, vertical: f32, sensitivity: f32) {
        self.rotation[0] -= horizontal * sensitivity;

        // dont let the player turn upside down
        if vertical > 0.0 && self.rotation[1] < PI * 0.5 {
            self.rotation[1] += vertical * sensitivity;
        } else if vertical < 0.0 && self.rotation[1] > -PI * 0.5 {
            self.rotation[1] += vertical * sensitivity;
        }
    }
}