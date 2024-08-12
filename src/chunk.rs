use crate::cube::Block::{DIRT, GRASS};
use crate::cube::{Block, Cube};

type ChunkData = [[[Option<Cube>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT];

pub const CHUNK_SIZE: usize = 8;
const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_FLOOR: usize = 9;
const CHUNK_MARGIN: f32 = 0.2;

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
    pub fn new_for_demo(corner: [f32; 2], z_offset: usize) -> Self {
        let mut cubes = [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT];
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                cubes[CHUNK_FLOOR - 2 + z_offset][i][j] = Some(Cube::new([corner[0] + i as f32, (CHUNK_FLOOR + z_offset) as f32 - 2. , corner[1] + j as f32], DIRT));
                cubes[CHUNK_FLOOR - 1 + z_offset][i][j] = Some(Cube::new([corner[0] + i as f32, (CHUNK_FLOOR + z_offset) as f32 - 1., corner[1] + j as f32], DIRT));
                cubes[CHUNK_FLOOR + z_offset][i][j] = Some(Cube::new([corner[0] + i as f32,     (CHUNK_FLOOR + z_offset) as f32, corner[1] + j as f32], GRASS));
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

    pub fn corner(&self) -> [f32; 2] {
        self.corner
    }

    /// Returns true if the position is in the chunk
    pub fn is_in(&self, pos: &[f32;3]) -> bool {
        // Note that in the received position, the 'y' (from the plane) position is actually the third value
        // of the vector...
        pos[0] >= self.corner[0] - CHUNK_MARGIN && pos[0] < (self.corner[0] + CHUNK_MARGIN + CHUNK_SIZE as f32) &&
            pos[2] >= self.corner[1] - CHUNK_MARGIN && pos[2] < (self.corner[1] + CHUNK_MARGIN + CHUNK_SIZE as f32)
    }
    
    /// Returns true if the position in the chunk is not part of a cube.
    /// The function does not check that the cube is chunk, and will crash if it is not.
    pub fn is_free(&self, pos: &[f32;3]) -> bool {
        let i_x = (pos[0] - self.corner[0]) as usize;
        let i_z = pos[1] as usize;
        let i_y = (pos[2] - self.corner[1]) as usize;
        i_z < CHUNK_HEIGHT && i_x < CHUNK_SIZE && i_y < CHUNK_SIZE && self.cubes[i_z][i_x][i_y].is_none()
    }
    
    pub fn print_all_cubes(&self) {
        for k in 0..CHUNK_HEIGHT {
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    if let Some(cube) = self.cubes[k][i][j] {
                        println!("* [{k},{i},{j}]{cube:?}")
                    }
                }
            }
        }
        
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
        
        // After 7.sssssss
        assert!(!chunk.is_in(&[4.,30.,7.5]));
    }
    
    #[test]
    fn test_free_check_1() {
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

    #[test]
    fn test_free_check_2() {
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(9, GRASS);
        assert!(chunk.is_free(&[4.0, 10.1, 4.0]));
    }

    #[test]
    fn test_free_check_3() {
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        assert!(!chunk.is_free(&[4.0, 0.1, 4.0]));
        assert!(!chunk.is_free(&[4.0, 0.5, 4.0]));
        assert!(!chunk.is_free(&[4.0, 0.9, 4.0]));
        assert!(chunk.is_free(&[4.0, 1.1, 4.0]));
        assert!(chunk.is_free(&[4.0, 1.2, 4.0]));
        assert!(chunk.is_free(&[4.0, 1.5, 4.0]));
    }

}