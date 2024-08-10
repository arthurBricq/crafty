use crate::cube::Block::GRASS;
use crate::cube::Cube;

const CHUNK_SIZE: usize = 8;
const CHUNK_HEIGHT: usize = 32;
const CHUNK_FLOOR: usize = 10;

/// A chunk is a (size * size * h) partition of the space that contains cubes
pub struct Chunk {
    cubes: [[[Option<Cube>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            cubes: [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT]
        }
    }

    /// Fills the chunk with a bluit-in world
    pub fn fill_for_demo(&mut self)  {
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                self.cubes[CHUNK_FLOOR][i][j] = Some(Cube::new([i as f32, CHUNK_FLOOR as f32, j as f32], GRASS));
            }
        }
    }

    pub fn cubes(&self) -> [[[Option<Cube>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT] {
        self.cubes
    }
}


/*

/// An iterator over all the cubes of a chunk
pub struct ChunkIterator<'a> {
    i: usize,
    j: usize,
    k: usize,
    chunk: &'a Chunk
}

impl<'a> ChunkIterator <'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self { i: 0, j: 0, k: 0, chunk }
    }
}

/// A chunk
impl<'a> Iterator for ChunkIterator<'a> {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
 */
