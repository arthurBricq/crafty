use crate::block_kind::Block;
use std::collections::HashMap;

/// A grid to encode the input of a crafting recipe
pub type CraftingGrid = [[Option<Block>; 3]; 3];

/// Structure responsible for the crafting recipes. For now, only its static
/// methods will be used, but in the end it will load configuration from a file,
/// so will be stateful
pub struct CraftingManager {}

impl CraftingManager {
    pub fn recipe(grid: &CraftingGrid) -> Option<Block> {
        let mut recipes: HashMap<CraftingGrid, Block> = HashMap::new();

        // TODO the rows are in the wrong order, be careful
        recipes.insert(
            [
                [None, Some(Block::OAKLOG), None],
                [None, Some(Block::COBBELSTONE), None],
                [None, Some(Block::COBBELSTONE), None],
            ],
            Block::SWORD,
        );

        recipes.get(grid).copied()
    }
}
