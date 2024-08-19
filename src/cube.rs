use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

/// The kind of cube
/// Each kind is associated with 3 textures: side, top & bottom.
#[derive(Clone, Copy, EnumIter, PartialEq, Debug, Serialize, Deserialize)]
pub enum Block {
    GRASS = 0,
    DIRT,
    COBBELSTONE,
    OAKLOG,
}

impl Block {
    fn file_name(&self) -> String {
        match self {
            Block::GRASS => "grass_block".to_string(),
            Block::DIRT => "dirt".to_string(),
            Block::COBBELSTONE => "cobblestone".to_string(),
            Block::OAKLOG => "oak_log".to_string(),
        }
    }

    /// Returns a list of all the textures to be loaded, in the proper order.
    pub fn get_texture_files() -> Vec<String> {
        let mut names = Vec::new();
        for block_kind in Block::iter() {
            let name = block_kind.file_name();
            names.push(name.clone() + "_side");
            names.push(name.clone() + "_top");
            names.push(name.clone() + "_bottom");
        }
        names
    }
}

/// Model of a cube in the 3D world.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Cube {
    position: [f32; 3],
    block: Block,
}


impl Cube {
    pub fn new(position: [f32; 3], block: Block) -> Self {
        Self { position, block }
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

    pub fn block_id(&self) -> u8 {
        self.block as u8
    }

    pub fn position(&self) -> [f32; 3] {
        self.position
    }
}
