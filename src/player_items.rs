use crate::block_kind::Block;
use crate::block_kind::Block::COBBELSTONE;

pub type Items = Vec<(Block, usize)>;

const CURRENT_ITEMS_SIZE: usize = 8;



/// Holds the items of a player.
pub struct PlayerItems {
    current_items: [Option<(Block, usize)>; CURRENT_ITEMS_SIZE],
    current_item: usize
}

impl PlayerItems {
    pub fn new() -> Self {
        Self {
            current_items: [None; CURRENT_ITEMS_SIZE],
            current_item: 0,
        }
    }
    
    pub fn get_current_items(&self) -> Items {
        self.current_items.iter()
            .filter(|item| item.is_some())
            .map(|item| item.unwrap()).collect()
    }

    pub fn get_current_block(&self) -> Option<Block> {
        if let Some((_, Some((block, _)))) = self.current_items.iter()
            .filter(|item| item.is_some())
            .enumerate()
            .find(|(i, _)| *i == self.current_item) {
            Some(block.clone())
        } else {
            None
        }
    }

    pub fn collect(&mut self, block: Block) {
        // Try to place the item in one of the containers
        for i in 0..CURRENT_ITEMS_SIZE {
            if let Some((b,c)) = self.current_items[i] {
                if b == block {
                    self.current_items[i] = Some((b, c + 1));
                    return;
                }
            }
        }

        // If we reach this place, it means that the player didn't have any item of this type.
        // So we try to place it in one of the empty containers
        for i in 0..CURRENT_ITEMS_SIZE {
            if self.current_items[i].is_none() {
                self.current_items[i] = Some((block, 1));
                return;
            }
        }
        // If we reach this place, it means the player can't collect this item...
    }

    pub fn consume(&mut self, block: Block) {
        for i in 0..CURRENT_ITEMS_SIZE {
            if let Some((b,c)) = self.current_items[i] {
                if b == block {
                    if c == 1 {
                        self.current_items[i] = None
                    } else {
                        self.current_items[i] = Some((b, c-1))
                    }
                    return;
                }
            }
        }
    }

    pub fn has_block(&self, block: Block) -> bool {
        self.current_items.iter()
            .any(|item| item.is_some_and(|(b, _)| b == block))
    }

    pub fn debug(&self) {
        println!("Debugging items...");
        for i in 0..CURRENT_ITEMS_SIZE {
            println!("{:?}", self.current_items[i])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::block_kind::Block::COBBELSTONE;
    use crate::player_items::PlayerItems;

    #[test]
    fn test_basic_operations() {
        let mut items = PlayerItems::new();

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
