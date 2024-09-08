use crate::block_kind::Block;

pub type ItemStack = (Block, usize);

const CURRENT_ITEMS_SIZE: usize = 8;
const MAX_ITEMS_IN_SLOT: usize = 64;


/// Holds the items of a player.
#[derive(Clone)]
pub struct PlayerItems {
    /// The items always displayed on the item bars
    bar_items: [Option<ItemStack>; CURRENT_ITEMS_SIZE],
    /// The items only visible when crafting
    inventory_items: [Option<ItemStack>; CURRENT_ITEMS_SIZE * 3],
    current_item: usize
}

impl PlayerItems {
    pub fn empty() -> Self {
        Self {
            bar_items: [None; CURRENT_ITEMS_SIZE],
            inventory_items: [None; CURRENT_ITEMS_SIZE * 3],
            current_item: 0,
        }
    }

    pub fn new(inventory_items: [Option<(Block, usize)>; CURRENT_ITEMS_SIZE * 3], current_items: [Option<(Block, usize)>; CURRENT_ITEMS_SIZE]) -> Self {
        Self {
            bar_items: current_items,
            inventory_items,
            current_item: 0
        }
    }

    pub fn get_bar_items(&self) -> Vec<ItemStack> {
        self.bar_items.iter()
            .filter(|item| item.is_some())
            .map(|item| item.unwrap())
            .collect()
    }

    pub fn get_bar_item(&self, index: usize) -> Option<ItemStack> {
        self.bar_items[index]
    }

    pub fn get_inventory_item(&self, index: usize) -> Option<ItemStack> {
        self.inventory_items[index]
    }

    pub fn take_bar_item(&mut self, index: usize) -> Option<Block> {
        Self::take_item(&mut self.bar_items[index])
    }

    pub fn take_inventory_item(&mut self, index: usize) -> Option<Block> {
        Self::take_item(&mut self.inventory_items[index])
    }

    pub fn put_bar_item(&mut self, index: usize, block: Block) -> bool {
        Self::put_item(&mut self.bar_items[index], block)
    }
    
    pub fn put_inventory_item(&mut self, index: usize, block: Block) -> bool {
        Self::put_item(&mut self.inventory_items[index], block)
    }
    
    pub fn get_current_block(&self) -> Option<Block> {
        if let Some((_, Some((block, _)))) = self.bar_items.iter()
            .filter(|item| item.is_some())
            .enumerate()
            .find(|(i, _)| *i == self.current_item) {
            Some(block.clone())
        } else {
            None
        }
    }

    pub fn collect(&mut self, block: Block) {

        fn place_in_collection(list: &mut [Option<ItemStack>], block: Block) -> bool {
            // First check if the item already exists in the list
            // If so, simply increase the counter
            for i in 0..list.len() {
                if let Some((b, count)) = list[i] {
                    if b == block && count < MAX_ITEMS_IN_SLOT {
                        list[i] = Some((b, count + 1));
                        return true;
                    }
                }
            }

            // Second, try to place the item in the first remaining slot
            for i in 0..list.len() {
                if list[i].is_none() {
                    list[i] = Some((block, 1));
                    return true;
                }
            }

            false
        }

        if !place_in_collection(&mut self.bar_items, block) {
            place_in_collection(&mut self.inventory_items, block);
        }

    }

    pub fn consume(&mut self, block: Block) {
        for i in 0..CURRENT_ITEMS_SIZE {
            if let Some((b,c)) = self.bar_items[i] {
                if b == block {
                    if c == 1 {
                        self.bar_items[i] = None
                    } else {
                        self.bar_items[i] = Some((b, c-1))
                    }
                    return;
                }
            }
        }
    }

    pub fn has_block(&self, block: Block) -> bool {
        self.bar_items.iter()
            .any(|item| item.is_some_and(|(b, _)| b == block))
    }

    pub fn debug(&self) {
        println!("Debugging items...");
        for i in 0..CURRENT_ITEMS_SIZE {
            println!("{:?}", self.bar_items[i])
        }
    }

    pub fn set_current_item(&mut self, current_item: usize) {
        self.current_item = current_item;
    }

    pub fn current_item(&self) -> usize {
        self.current_item
    }

    fn take_item(itemstack: &mut Option<ItemStack>) -> Option<Block> {
        let mut ret: Option<Block> = None;
        
        *itemstack = if let Some((block, count)) = itemstack {
            if *count > 0 {
                ret = Some(*block);
            }
            
            if *count > 1 {
                Some((*block, *count - 1))
            } else {
                None
            }
        } else {
            None
        };

        ret
    }

    fn put_item(itemstack: &mut Option<ItemStack>, block: Block) -> bool {
        let mut success: bool = false;
        *itemstack = match itemstack {
            Some((block2, count)) => {
                if *block2 == block {
                    // TODO put stack limit
                    success = true;
                    Some((*block2, *count + 1))
                } else {
                    Some((*block2, *count))
                }
            },
            None => {
                success = true;
                Some((block, 1))
            }
        };

        success
    }
}

#[cfg(test)]
mod tests {
    use crate::block_kind::Block::COBBELSTONE;
    use crate::player_items::PlayerItems;

    #[test]
    fn test_basic_operations() {
        let mut items = PlayerItems::empty();

        // At first, there is simply no block to add
        assert_eq!(items.get_current_block(), None);

        // The player collects 2 rocks
        items.collect(COBBELSTONE);
        items.debug();

        items.collect(COBBELSTONE);

        // He can place two (and only two) cobbelstones
        assert_eq!(items.get_current_block(), Some(COBBELSTONE));
        items.consume(COBBELSTONE);
        assert_eq!(items.get_current_block(), Some(COBBELSTONE));
        items.consume(COBBELSTONE);

        items.debug();

        // After consuming 2 stones, we don't have anymore cubes to place
        assert_eq!(items.get_current_block(), None);
    }
}
