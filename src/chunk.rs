use crate::cube::Block::{DIRT, GRASS};
use crate::cube::{Block, Cube};

type ChunkData = [[[Option<Cube>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT];

pub const CHUNK_SIZE: usize = 8;
const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_FLOOR: usize = 10;

/// A chunk is a (size * size * h) partition of the space that contains cubes
///
/// * A chunk is described by the position of one of his corner: the one with the lowest x-z value
///
/// * The chunk owns the cube that it contains and is responsible for properly constructing / modifying them.
///   As a consequence, it is the position in the `ChunkData` field that encodes the position of each cube.
///
pub struct Chunk {
    cubes: ChunkData,
    corner: [f32; 2]
}

impl Chunk {
    pub fn new(corner: [f32;2]) -> Self {
        Self {
            cubes: [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT],
            corner,
        }
    }

    /// Fills the chunk with a bluit-in world
    pub fn new_for_demo(corner: [f32; 2], z_offset: f32) -> Self {
        let mut cubes = [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT];
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                cubes[CHUNK_FLOOR - 2][i][j] = Some(Cube::new([corner[0] + i as f32, CHUNK_FLOOR as f32 - 2. + z_offset, corner[1] + j as f32], DIRT));
                cubes[CHUNK_FLOOR - 1][i][j] = Some(Cube::new([corner[0] + i as f32, CHUNK_FLOOR as f32 - 1. + z_offset, corner[1] + j as f32], DIRT));
                cubes[CHUNK_FLOOR][i][j] = Some(Cube::new([corner[0] + i as f32, CHUNK_FLOOR as f32 + z_offset, corner[1] + j as f32], GRASS));
            }
        }
        Self { cubes, corner }
    }
    
    /// Fills a full layer of the chunk with one kind of block
    pub fn fill_layer(&mut self, h: usize, kind: Block) {
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                self.cubes[h][i][j] = Some(
                    Cube::new([self.corner[0] + i as f32, h as f32, self.corner[1] + j as f32], kind));
            }
        }
    }

    pub fn cubes(&self) -> &ChunkData {
        &self.cubes
    }

    /// Returns true if the position is in the chunk
    pub fn is_in(&self, pos: &[f32;3]) -> bool {
        // Note that in the received position, the 'y' (from the plane) position is actually the third value
        // of the vector...
        pos[0] >= self.corner[0] && pos[0] < (self.corner[0] + CHUNK_SIZE as f32) &&
            pos[2] >= self.corner[1] && pos[2] < (self.corner[1] + CHUNK_SIZE as f32)
    }
    
    /// Returns true if the position in the chunk is not part of a cube.
    /// The function does not check that the cube is chunk, and will crash if it is not.
    pub fn is_free(&self, pos: &[f32;3]) -> bool {
        let i_x = (pos[0] - self.corner[0]) as usize;
        let i_z = pos[1] as usize;
        let i_y = (pos[2] - self.corner[1]) as usize;
        self.cubes[i_z][i_x][i_y].is_none()
    }
    

}

#[cfg(test)]
mod tests {
    use crate::chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE};
    use crate::cube::Block::GRASS;

    #[test]
    fn test_bounding_area() {
        let chunk = Chunk::new([0., 0.]);
        assert!(chunk.is_in(&[0.,0.,0.]));
        assert!(chunk.is_in(&[1.,30.,1.]));
        assert!(!chunk.is_in(&[-1.,30.,1.]));
    }
    
    #[test]
    fn test_free_check() {
        let mut chunk = Chunk::new([0., 0.]);
        
        // First, assert positions are free when there are no cubes at all
        for k in 0..CHUNK_HEIGHT {
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    assert!(chunk.is_free(&[i as f32,k as f32,j as f32]));
                }
            }
        }
        
        // Fill the 10-th layer
        chunk.fill_layer(10, GRASS);
        
        // Assert that only positions on the 10.th layer are not free
        for k in 0..CHUNK_HEIGHT {
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    if k != 10 {
                        assert!(chunk.is_free(&[i as f32,k as f32,j as f32]));
                    } else {
                        assert!(!chunk.is_free(&[i as f32,k as f32,j as f32]));
                    }
                }
            }
        }
    }

}