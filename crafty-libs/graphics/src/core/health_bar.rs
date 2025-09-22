use primitives::color::Color::Red;
use primitives::opengl::rectangle::RectInstance;

pub struct HealthBar {
    health: u8,
    rects: Vec<RectInstance>,
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

    pub fn rects(&self) -> Vec<RectInstance> {
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
            let hp = RectInstance::square_from_corner(
                X0 + n as f32 * (HP_SIDE + INNER_MARGIN),
                Y0,
                HP_SIDE,
                self.aspect_ratio,
                Red,
            );
            self.rects.push(hp);
        }
    }
}
