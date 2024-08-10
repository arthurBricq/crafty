use crate::cube::Block::{DIRT, GRASS};
use crate::cube::{Cube, CubeAttr};

pub struct World {
    cubes: Vec<Cube>
}

impl World {
    pub fn new() -> Self {
        let mut cubes = Vec::new();

        for i in 0..1000 {
            cubes.push(Cube::new([-i as f32*2., 0., 0.], GRASS));
            cubes.push(Cube::new([-i as f32*2., 0., 1.], GRASS));
            cubes.push(Cube::new([-i as f32*2., 0., -1.], GRASS));
            cubes.push(Cube::new([-i as f32*2., 0., 2.], GRASS));
            cubes.push(Cube::new([-i as f32*2., 0., -2.], GRASS));

            cubes.push(Cube::new([-i as f32*2., -1., 0.], DIRT));
            cubes.push(Cube::new([-i as f32*2., -1., 1.], DIRT));
            cubes.push(Cube::new([-i as f32*2., -1., -1.], DIRT));
            cubes.push(Cube::new([-i as f32*2., -1., 5.], DIRT));
            cubes.push(Cube::new([-i as f32*2., -1., -5.], DIRT));
        }

        Self {
            cubes
        }
    }

    pub fn cubes(&self) -> &Vec<Cube> {
        &self.cubes
    }
    
    pub fn get_cube_attributes(&self) -> Vec<CubeAttr> {
        let mut positions: Vec<CubeAttr> = Vec::new();
        for cube in self.cubes() {
            positions.push(CubeAttr::new(cube.model_matrix(), cube.block_id()));
        }
        positions
    }
}