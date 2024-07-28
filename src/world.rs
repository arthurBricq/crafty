use crate::cube::Cube;

pub struct World {
    cubes: Vec<Cube>
}

impl World {
    pub fn new() -> Self {
        Self {
            cubes: vec![
                Cube::new([0., 0., 0.]),
                Cube::new([0., 0., 2.]),
                Cube::new([-2., 0., 0.]),
                Cube::new([-2., 0., 2.])
            ]
        }
    }

    pub fn cubes(&self) -> &Vec<Cube> {
        &self.cubes
    }
}