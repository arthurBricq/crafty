pub struct Health {
    health: u8,
}

impl Health {
    pub fn new(health: u8) -> Self {
        Self { health }
    }

    pub fn alive(&self) -> bool {
        self.health > 0
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn damage(&mut self, strength: u8) {
        self.health = if self.health >= strength {
            self.health - strength
        } else {
            0
        };
    }
}
