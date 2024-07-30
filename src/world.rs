use crate::cube::Cube;

pub struct World {
    cubes: Vec<Cube>
}

impl World {
    pub fn new() -> Self {
        let mut cubes = Vec::new();
        
        for i in 0..100 {
            cubes.push(Cube::new([-i as f32*2., 0., 0.]));
            cubes.push(Cube::new([-i as f32*2., 0., 2.]));
            cubes.push(Cube::new([-i as f32*2., 0., -2.]));
            cubes.push(Cube::new([-i as f32*2., 0., 5.]));
            cubes.push(Cube::new([-i as f32*2., 0., -5.]));

            cubes.push(Cube::new([-i as f32*2., 2., 0.]));
            cubes.push(Cube::new([-i as f32*2., 2., 2.]));
            cubes.push(Cube::new([-i as f32*2., 2., -2.]));
            cubes.push(Cube::new([-i as f32*2., 2., 5.]));
            cubes.push(Cube::new([-i as f32*2., 2., -5.]));

            cubes.push(Cube::new([-i as f32*2., 4., 0.]));
            cubes.push(Cube::new([-i as f32*2., 4., 2.]));
            cubes.push(Cube::new([-i as f32*2., 4., -2.]));
            cubes.push(Cube::new([-i as f32*2., 4., 5.]));
            cubes.push(Cube::new([-i as f32*2., 4., -5.]));
        }
        
        Self {
            cubes
        }
    }

    pub fn cubes(&self) -> &Vec<Cube> {
        &self.cubes
    }
}