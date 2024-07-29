pub struct Camera {
    camera_pos: [f32; 3],
    camera_dir: [f32; 3],
    camera_up: [f32; 3]
}

impl Camera {
    pub fn new() -> Self {
        Self {
            camera_pos: [2.0, 0.0, 0.0],
            camera_dir: [-1.0, 0.0, 0.0],
            camera_up: [0.0, 1.0, 0.0]
        }
    }

    /// Returns the view matrix, from the given camera parameters
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.camera_dir;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [self.camera_up[1] * f[2] - self.camera_up[2] * f[1],
            self.camera_up[2] * f[0] - self.camera_up[0] * f[2],
            self.camera_up[0] * f[1] - self.camera_up[1] * f[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0]];

        let p = [-self.camera_pos[0] * s_norm[0] - self.camera_pos[1] * s_norm[1] - self.camera_pos[2] * s_norm[2],
            -self.camera_pos[0] * u[0] - self.camera_pos[1] * u[1] - self.camera_pos[2] * u[2],
            -self.camera_pos[0] * f[0] - self.camera_pos[1] * f[1] - self.camera_pos[2] * f[2]];

        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    pub fn forward(&mut self) {
        self.camera_pos[0] += 1.;
    }

    pub fn backward(&mut self) {
        self.camera_pos[0] -= 1.;
    }

    pub fn up(&mut self) {
        self.camera_pos[1] += 1.;
    }

    pub fn down(&mut self) {
        self.camera_pos[1] -= 1.;
    }
    pub fn right(&mut self) {
        self.camera_pos[2] -= 1.;
    }

    pub fn left(&mut self) {
        self.camera_pos[2] += 1.;
    }
}
