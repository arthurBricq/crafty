use primitives::color::Color::Red;
use crate::renderer::RectRenderData;

pub struct HealthBar {
    health: u8,
    rects: Vec<RectRenderData>,
    aspect_ratio: f32,
}

impl HealthBar {
    pub fn new(health: u8, aspect_ratio: f32) -> Self {
        let mut menu = Self {
            health,
            rects: Vec::new(),
            aspect_ratio,
        };

        menu.update();

        menu
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
        self.update();
    }

    pub fn set_health(&mut self, health: u8) {
        self.health = health;
        self.update();
    }

    pub fn rects(&self) -> Vec<RectRenderData> {
        self.rects.clone()
    }

    fn update(&mut self) {
        println!("there are {}", self.health);
        self.rects = Vec::new();

        const X0: f32 = 0.12 - 1.;
        const Y0: f32 = 0.46 - 1.;
        const INNER_MARGIN: f32 = 0.01;
        const HP_SIDE: f32 = 0.05;

        // Add the health points
        for n in 0..self.health {
            let hp_u = X0 + n as f32 * (HP_SIDE + INNER_MARGIN) + HP_SIDE / 2.;
            let hp_v = Y0 + HP_SIDE / 2.;
            let hp_w = HP_SIDE;  // Full width
            let hp_h = HP_SIDE;  // Full height
            self.rects.push(RectRenderData {
                u: hp_u,
                v: hp_v,
                w: hp_w,
                h: hp_h,
                color: Red,
                is_font: false,
                font_coords: None,
                block_id: None,
            });
        }
    }
}
