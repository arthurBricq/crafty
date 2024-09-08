use crate::chunk::CHUNK_SIZE;
use std::collections::HashSet;

const VISIBLE_CHUNKS: i32 = 4;

/// A struct in charge of keeping track of the chunks loaded by each players.
pub struct WorldDispatcher {
    // Note for all the following attributes
    // -------------------------------------
    // The player ids are created SEQUENTIALLY, from 0 to N
    // Therefore, we can access the data of 1 player by accessible its 'userid'
    // This is done for optimization.

    /// The chunks loaded by this player
    player_chunks: Vec<HashSet<(i32, i32)>>,

    /// The current position of this player
    /// We only care about the position on the 'xy' plane.
    player_current_chunk: Vec<(i32, i32)>
}

impl WorldDispatcher {

    pub fn new() -> Self {
        Self {
            player_chunks: Vec::new(),
            player_current_chunk: vec![]
        }
    }

    pub fn register_player(&mut self, _id: usize) {
        self.player_chunks.push(HashSet::new());
        self.player_current_chunk.push((-10, -10))
    }

    pub fn logout(&mut self, id: usize) {
        self.player_chunks[id] = HashSet::new();
    }

    /// Updates the position and returns a list of new chunks to be loaded
    pub fn update_position(&mut self, id: usize, pos: (f32, f32)) -> Option<(HashSet<(i32, i32)>, HashSet<(i32, i32)>)> {
        // Compute the current chunk
        let s = CHUNK_SIZE as f32;
        let current_chunk = ((pos.0 - pos.0 % s) as i32, (pos.1 - pos.1 % s) as i32);

        // If the player has changed chunk, then we send the new chunks that are further away
        if current_chunk != self.player_current_chunk[id] {
            // Compute the necessary chunks
            let chunks_to_see = Self::get_visible_chunk(current_chunk);
            
            // Compute the diff
            let chunks_to_send = &chunks_to_see - &self.player_chunks[id];
            let chunks_to_delete = &self.player_chunks[id] - &chunks_to_see;
            
            // Update the information about this player...
            self.player_current_chunk[id] = current_chunk;
            chunks_to_send.iter().for_each(|chunk| {
                self.player_chunks[id].insert(chunk.clone());
            });
            
            return Some((chunks_to_send, chunks_to_delete))
        }
        None
    }

    // TODO output of this function can be memoized
    fn get_visible_chunk(from: (i32, i32)) -> HashSet<(i32, i32)> {
        let mut chunks = HashSet::new();
        for i in -VISIBLE_CHUNKS..VISIBLE_CHUNKS {
            for j in -VISIBLE_CHUNKS..VISIBLE_CHUNKS {
                chunks.insert((from.0 + i * CHUNK_SIZE as i32,from.1 + j * CHUNK_SIZE as i32));
            }
        }
        chunks
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::CHUNK_SIZE;
    use crate::server::world_dispatcher::{WorldDispatcher, VISIBLE_CHUNKS};

    #[test]
    fn test_basic_scenario() {
        let mut dispatcher = WorldDispatcher::new();
        dispatcher.register_player(0);
        
        // Initially, the server sends the full grid around the player
        let (to_send, to_delete) = dispatcher.update_position(0, (0., 0.)).unwrap();
        assert_eq!(to_send.len(), (4 * VISIBLE_CHUNKS * VISIBLE_CHUNKS) as usize);
        assert_eq!(to_delete.len(), 0);
        
        // If you remain in the same position, then you're fine
        assert!(dispatcher.update_position(0, (1., 1.)).is_none());
        
        // If you go one chunk more torwards the right, then you'll load 4 new chunks
        let (to_send, to_delete) = dispatcher.update_position(0, (CHUNK_SIZE as f32 + 1., 1.)).unwrap();
        assert_eq!(to_send.len(), 2 * VISIBLE_CHUNKS as usize);
        assert_eq!(to_delete.len(), 2 * VISIBLE_CHUNKS as usize);
        
    }
}
