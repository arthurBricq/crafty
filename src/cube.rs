use crate::block_kind::Block;
use crate::vector::Vector3;

/// Model of a cube in the 3D world.
#[derive(Clone, Copy, Debug)]
pub struct Cube {
    position: Vector3,
    block: Block,
    is_visible: bool
}

impl Cube {
    pub fn new(position: [f32; 3], block: Block, visible: bool) -> Self {
        Self { position: Vector3::newf(position), block, is_visible: visible }
    }

    pub fn model_matrix(&self) -> [[f32; 4]; 4] {
        // TODO As you can see, I added 0.5 at each cube model
        //      It's because I was lazy to edit all the values in `VERTICES` of +0.5, but
        //      it would be nice to do it eventually :)
        [
            [1.00, 0.0, 0.0, 0.0],
            [0.0, 1.00, 0.0, 0.0],
            [0.0, 0.0, 1.00, 0.0],
            [self.position[0] + 0.5, self.position[1] + 0.5, self.position[2] + 0.5, 1.0f32]
        ]
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
    
    pub fn neighbors_positions(&self) -> [Vector3; 6] {
        [
            self.position + Vector3::unit_x(),
            self.position - Vector3::unit_x(),
            self.position + Vector3::unit_y(),
            self.position - Vector3::unit_y(),
            self.position + Vector3::unit_z(),
            self.position - Vector3::unit_z(),
        ]
    }

    pub fn set_is_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}
