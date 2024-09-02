use crate::block_kind::Block;
use crate::block_kind::Block::{DIRT, GRASS};
use crate::cube::Cube;
use crate::vector::Vector3;
use crate::world_serializer::{get_serialize_container, serialize_one_chunk, SerializedWorld};
use strum::IntoEnumIterator;

type ChunkData = [[[Option<Cube>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT];
pub type CubeIndex = (usize, usize, usize);

pub const CHUNK_SIZE: usize = 8;
const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_FLOOR: usize = 9;

/// A chunk is a (size * size * h) partition of the space that contains cubes
///
/// * A chunk is described by the position of one of his corner: the one with the lowest x-z value
///
/// * The chunk owns the cube that it contains and is responsible for properly constructing / modifying them.
///   As a consequence, it is the position in the `ChunkData` field that encodes the position of each cube.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    cubes: ChunkData,
    corner: [f32; 2],
}

impl Chunk {
    pub fn new(corner: [f32; 2]) -> Self {
        Self {
            cubes: [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT],
            corner,
        }
    }

    pub fn cubes(&self) -> &ChunkData {
        &self.cubes
    }

    pub fn corner(&self) -> [f32; 2] {
        self.corner
    }


    /// Returns an iterator over all the positions of the chunk
    pub fn flattened_iter(&self) -> impl Iterator<Item = &Option<Cube>> {
        self.cubes.iter()
            .flat_map(|matrix_2d| matrix_2d.iter())
            .flat_map(|row| row.iter())
    }

    /// Fills the chunk with a bluit-in world
    pub fn new_for_demo(corner: [f32; 2], z_offset: i32) -> Self {
        let mut cubes = [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_HEIGHT];
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                cubes[(CHUNK_FLOOR as i32 - 2 + z_offset) as usize][i][j] = Some(Cube::new([corner[0] + i as f32, (CHUNK_FLOOR as i32 + z_offset) as f32 - 2., corner[1] + j as f32], DIRT, 0));
                cubes[(CHUNK_FLOOR as i32 - 1 + z_offset) as usize][i][j] = Some(Cube::new([corner[0] + i as f32, (CHUNK_FLOOR as i32 + z_offset) as f32 - 1., corner[1] + j as f32], DIRT, 0));
                cubes[(CHUNK_FLOOR as i32 + z_offset) as usize][i][j] = Some(Cube::new([corner[0] + i as f32, (CHUNK_FLOOR as i32 + z_offset) as f32, corner[1] + j as f32], GRASS, 0));
            }
        }
        Self { cubes, corner }
    }

    /// Fills a full layer of the chunk with one kind of block
    pub fn fill_layer(&mut self, h: usize, kind: Block) {
        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE {
                self.cubes[h][i][j] = Some(
                    Cube::new([self.corner[0] + i as f32, h as f32, self.corner[1] + j as f32], kind, 0));
            }
        }
    }

    pub fn destroy_cube(&mut self, at: Vector3) {
        let (i_z, i_x, i_y) = self.get_indices(&at);
        let in_bound = i_z < CHUNK_HEIGHT && i_x < CHUNK_SIZE && i_y < CHUNK_SIZE;
        if in_bound {
            self.cubes[i_z][i_x][i_y] = None
        }
    }

    pub fn add_cube(&mut self, at: Vector3, block: Block, neighbors: u8) -> Option<&mut Cube> {
        let (i_z, i_x, i_y) = self.get_indices(&at);
        let in_bound = i_z < CHUNK_HEIGHT && i_x < CHUNK_SIZE && i_y < CHUNK_SIZE;
        if in_bound {
            self.cubes[i_z][i_x][i_y] = Some(Cube::new(at.as_array(), block, neighbors));
            self.cubes[i_z][i_x][i_y].as_mut()
        } else {
            None
        }
    }

    /// Returns true if the position is in the chunk
    pub fn is_in(&self, pos: &Vector3) -> bool {
        // Note that in the received position, the 'y' (from the plane) position is actually the third value
        // of the vector...
        pos[0] >= self.corner[0] && pos[0] < (self.corner[0] + CHUNK_SIZE as f32) &&
            pos[2] >= self.corner[1] && pos[2] < (self.corner[1] + CHUNK_SIZE as f32)
    }


    /// Returns true if the position in the chunk is not part of a cube.
    /// The function does not check that the cube is chunk, and will crash if it is not.
    pub fn is_position_free(&self, pos: &Vector3) -> bool {
        let (i_z, i_x, i_y) = self.get_indices(pos);
        let in_bound = i_z < CHUNK_HEIGHT && i_x < CHUNK_SIZE && i_y < CHUNK_SIZE;
        let result = !in_bound || self.cubes[i_z][i_x][i_y].is_none();
        result
    }

    pub fn is_position_free_falling(&self, pos: &Vector3) -> bool {
        // We simply check if the cube below the player is occupied.
        self.is_position_free(&Vector3::new(pos[0], pos[1] - 1., pos[2]))
    }

    /// Goes through all the cubes that are strictly inside the chunk and compute whether they have
    /// a free neighbors.
    pub fn compute_visible_cubes(&mut self) {
        for k in 1..CHUNK_HEIGHT-1 {
            for i in 1..CHUNK_SIZE-1 {
                for j in 1..CHUNK_SIZE-1 {
                    if self.cubes[k][i][j].is_some() {
                        // Each cube has 6 potential neighbors.
                        // We set the cube as not visible if all the 6 neighbors are not full
                        // If either one is none, the cube must be visible.
                        let mut count = 0;
                        if self.cubes[k-1][i][j].is_some() { count += 1 }
                        if self.cubes[k+1][i][j].is_some() { count += 1 }
                        if self.cubes[k][i-1][j].is_some() { count += 1 }
                        if self.cubes[k][i+1][j].is_some() { count += 1 }
                        if self.cubes[k][i][j-1].is_some() { count += 1 }
                        if self.cubes[k][i][j+1].is_some() { count += 1 }
                        self.cubes[k][i][j].as_mut().unwrap().set_n_neighbors(count);
                    }
                }
            }
        }
    }

    /// Returns the list of the index of all the cubes located at the border of the chunk.
    pub fn border(&self) -> Vec<CubeIndex> {
        let mut to_return = Vec::new();
        for k in 0..CHUNK_HEIGHT {
            for i in 0..CHUNK_SIZE {
                if self.cubes[k][i][0].is_some() {
                    to_return.push((k, i, 0));
                }
                if self.cubes[k][i][CHUNK_SIZE-1].is_some() {
                    to_return.push((k, i, CHUNK_SIZE-1));
                }
            }
            for j in 0..CHUNK_SIZE {
                if self.cubes[k][0][j].is_some() {
                    to_return.push((k, 0, j));
                }
                if self.cubes[k][CHUNK_SIZE-1][j].is_some() {
                    to_return.push((k, CHUNK_SIZE-1, j));
                }
            }
        }
        
        // You also have to provide all the cubes in the bottom-most layer
        for i in 1..CHUNK_SIZE-1 {
            for j in 1..CHUNK_SIZE-1 {
                to_return.push((0, i, j));
            }
        }
        
        to_return
    }

    fn get_indices(&self, pos: &Vector3) -> CubeIndex {
        let i_x = (pos[0] - self.corner[0]) as usize;
        let i_z = pos[1] as usize;
        let i_y = (pos[2] - self.corner[1]) as usize;
        (i_z, i_x, i_y)
    }
    
    pub fn cube_at_index(&self, index: CubeIndex) -> Option<&Cube> {
        let (k,i,j) = index;
        self.cubes[k][i][j].as_ref()
    }

    pub fn cube_at_index_mut(&mut self, index: CubeIndex) -> Option<&mut Cube> {
        let (k,i,j) = index;
        self.cubes[k][i][j].as_mut()
    }
    
    pub fn cube_at(&self, pos: &Vector3) -> Option<&Cube> {
        self.cube_at_index(self.get_indices(pos))
    }
    
    pub fn cube_at_mut(&mut self, pos: &Vector3) -> Option<&mut Cube> {
        self.cube_at_index_mut(self.get_indices(pos))
    }

    pub fn visible_cube_count(&self) -> usize {
        let mut count = 0;
        for k in 0..CHUNK_HEIGHT {
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    if let Some(c) = &self.cubes[k][i][j] {
                        if c.is_visible() {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn print_all_cubes(&self) {
        for k in 0..1 {
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    if let Some(cube) = self.cubes[k][i][j] {
                        println!("* [{k},{i},{j}]{cube:?}")
                    }
                }
            }
        }
    }

    pub fn to_json(&self) -> String {
        let mut all_cubes = get_serialize_container();
        serialize_one_chunk(&mut all_cubes, self);
        let world = SerializedWorld {
            chunk_corners: vec![self.corner],
            cubes_by_kind: all_cubes
        };
        serde_json::to_string(&world).unwrap()
    }

    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        let serialized_world: SerializedWorld = serde_json::from_str(data)?;
        let mut chunk = Chunk::new(serialized_world.chunk_corners[0]);
        for block_kind in Block::iter() {
            let cubes = serialized_world.cubes_by_kind.get(&block_kind).unwrap();
            for cube_data in cubes {
                let x = cube_data[0] as f32;
                let y = cube_data[1] as f32;
                let z = cube_data[2] as f32;
                let neighbors = cube_data[3] as u8;
                chunk.add_cube(Vector3::new(x,y,z), block_kind, neighbors);
            }
        }
        Ok(chunk)
    }

}

#[cfg(test)]
mod tests {
    use crate::block_kind::Block::GRASS;
    use crate::chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE};
    use crate::vector::Vector3;

    #[test]
    fn test_bounding_area() {
        let chunk = Chunk::new([0., 0.]);
        assert!(chunk.is_in(&Vector3::new(0., 0., 0.)));
        assert!(chunk.is_in(&Vector3::new(1., 30., 1.)));
        assert!(!chunk.is_in(&Vector3::new(-1., 30., 1.)));
        assert!(chunk.is_in(&Vector3::new(4., 30., 7.5)));
        assert!(!chunk.is_in(&Vector3::new(4., 30., 8.5)));
        assert!(!chunk.is_in(&Vector3::new(9., 30., 4.5)));
    }
    
    #[test]
    fn test_is_in() {
        let chunk = Chunk::new([0., -(CHUNK_SIZE as f32)]);
        assert!(!chunk.is_in(&Vector3::new(0., 0., 0.)));
        assert!(!chunk.is_in(&Vector3::new(0., 0., 3.)));
    }

    #[test]
    fn test_free_check_1() {
        let mut chunk = Chunk::new([0., 0.]);

        // First, assert positions are free when there are no cubes at all
        for k in 0..CHUNK_HEIGHT {
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    assert!(chunk.is_position_free(&Vector3::new(i as f32, k as f32, j as f32)));
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
                        assert!(chunk.is_position_free(&Vector3::new(i as f32, k as f32, j as f32)));
                    } else {
                        assert!(!chunk.is_position_free(&Vector3::new(i as f32, k as f32, j as f32)));
                    }
                }
            }
        }
    }

    #[test]
    fn test_free_check_2() {
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(9, GRASS);
        assert!(chunk.is_position_free(&Vector3::new(4.0, 10.1, 4.0)));
    }

    #[test]
    fn test_free_check_3() {
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        assert!(!chunk.is_position_free(&Vector3::new(4.0, 0.1, 4.0)));
        assert!(!chunk.is_position_free(&Vector3::new(4.0, 0.5, 4.0)));
        assert!(!chunk.is_position_free(&Vector3::new(4.0, 0.9, 4.0)));
        assert!(chunk.is_position_free(&Vector3::new(4.0, 1.1, 4.0)));
        assert!(chunk.is_position_free(&Vector3::new(4.0, 1.2, 4.0)));
        assert!(chunk.is_position_free(&Vector3::new(4.0, 1.5, 4.0)));
    }

    #[test]
    fn test_free_fall_1() {
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        let x = 4.;
        let y = 4.;
        assert!(!chunk.is_position_free_falling(&Vector3::new(x, 0., y)));
        assert!(chunk.is_position_free_falling(&Vector3::new(x, 2., y)));
        assert!(chunk.is_position_free_falling(&Vector3::new(x, 3., y)));
    }



    #[test]
    fn test_visible_cube_in_one_chunnk() {
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        chunk.fill_layer(1, GRASS);
        chunk.fill_layer(2, GRASS);

        // Before computing visible cube, we make sure that all cubes are visible
        assert_eq!(chunk.visible_cube_count(), 3 * CHUNK_SIZE * CHUNK_SIZE);

        // After the computation, there must be many less
        chunk.compute_visible_cubes();
        assert!(chunk.visible_cube_count() < 3 * CHUNK_SIZE * CHUNK_SIZE);

        // In this case, we know the actual number of cubes not visible.
        assert_eq!(chunk.visible_cube_count(), 3 * CHUNK_SIZE * CHUNK_SIZE - (CHUNK_SIZE - 2) * (CHUNK_SIZE - 2));
    }

    #[test]
    fn test_chunk_persistence() {
        let chunk = Chunk::new_for_demo([3., 4.], 5);
        let serialized = chunk.to_json();
        let reconstructed = Chunk::from_json(serialized.as_str());
        assert_eq!(chunk, reconstructed);
    }
}
