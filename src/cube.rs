use crate::block_kind::Block;
use crate::vector::Vector3;

/// Model of a cube in the 3D world.
#[derive(Clone, Copy, Debug)]
pub struct Cube {
    position: Vector3,
    block: Block,
    n_neighbors: u8
}

impl Cube {
    pub fn new(position: [f32; 3], block: Block, neighbors: u8) -> Self {
        Self {
            position: Vector3::newf(position),
            block,
            n_neighbors: neighbors
        }
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn block_id(&self) -> u8 {
        self.block as u8
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }
    
    pub fn neighbors_positions(position: Vector3) -> [Vector3; 6] {
        [
            position + Vector3::unit_x(),
            position - Vector3::unit_x(),
            position + Vector3::unit_y(),
            position - Vector3::unit_y(),
            position + Vector3::unit_z(),
            position - Vector3::unit_z(),
        ]
    }

    pub fn is_visible(&self) -> bool {
        self.n_neighbors < 6
    }

    pub fn add_neighhor(&mut self) {
        if self.n_neighbors < 6 {
            self.n_neighbors += 1;
        }
    }

    pub fn remove_neighbor(&mut self) {
        if self.n_neighbors > 0 {
            self.n_neighbors -= 1;
        }
    }

    pub fn set_n_neighbors(&mut self, n_neighbors: u8) {
        self.n_neighbors = n_neighbors;
    }

    pub fn n_neighbors(&self) -> u8 {
        self.n_neighbors
    }
}
