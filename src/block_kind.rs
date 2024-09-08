use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

/// The kind of cube
/// Each kind is associated with 3 textures: side, top & bottom.
#[derive(Clone, Copy, EnumIter, PartialEq, Debug, Serialize, Deserialize, Hash, Eq)]
pub enum Block {
    GRASS = 0,
    DIRT,
    COBBELSTONE,
    OAKLOG,
    OAKLEAVES,
    WATER
}

impl Block {
    fn file_name(&self) -> String {
        match self {
            Block::GRASS => "grass".to_string(),
            Block::DIRT => "dirt".to_string(),
            Block::COBBELSTONE => "cobblestone".to_string(),
            Block::OAKLOG => "oak_log".to_string(),
            Block::OAKLEAVES => "oak_leaves".to_string(),
            Block::WATER => "water".to_string(),
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
