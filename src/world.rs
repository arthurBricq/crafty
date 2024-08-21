use crate::actions::Action;
use crate::chunk::{Chunk, CubeIndex, CHUNK_FLOOR, CHUNK_SIZE};
use crate::graphics::cube::CubeAttr;
use crate::vector::Vector3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use crate::block_kind::Block;
use crate::block_kind::Block::{DIRT, GRASS};
use crate::cube::Cube;
use crate::world_generation::perlin::PerlinNoise;

pub struct World {
    /// The list of the chunks currently being displayed
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> Self {
        let chunks = Vec::new();
        Self { chunks }
    }

    pub fn fill_for_demo(&mut self) {
        let s = CHUNK_SIZE as f32;
        self.chunks.push(Chunk::new_for_demo([0., 0.], 0));
        self.chunks.push(Chunk::new_for_demo([s, 0.], 2));
        self.chunks.push(Chunk::new_for_demo([0., -s], 2));
        self.chunks.push(Chunk::new_for_demo([0., s], 2));
        self.chunks.push(Chunk::new_for_demo([-s, 0.], 0));
        self.chunks.push(Chunk::new_for_demo([-2. * s, 0.], 0));
    }

    /// Creates a basic, flat world. For now this is a simple, flat
    /// grassland, extending `nchunks` in each direction.
    ///
    /// 'n_chunks': number of chunks the flat lands extends in any
    /// direction; i.e., (2nchunks + 1) x (2nchunks + 1) chunks will
    /// be created
    pub fn create_new_flat_world(n_chunks: i32) -> Self {
        let s = CHUNK_SIZE as f32;
        let mut chunks = vec![];

        // Yes this is slow, but it will be fine for now
        for i in -n_chunks..n_chunks + 1 {
            for j in -n_chunks..n_chunks + 1 {
                let mut chunk = Chunk::new([i as f32 * s, j as f32 * s]);
                for k in 0..CHUNK_FLOOR {
                    chunk.fill_layer(k, DIRT);
                }
                chunk.fill_layer(CHUNK_FLOOR, GRASS);
                chunks.push(chunk);
            }
        }

        let mut world = Self { chunks };
        world.compute_visible_cubes();
        world
    }

    /// Creates a simple world with hills. A single octave Perlin noise is used,
    /// so don't expect anything fancy.
    pub fn create_new_random_world(n_chunks: i32) -> Self {
	let mut noise = PerlinNoise::new(42, 32.);

        let s = CHUNK_SIZE as f32;
        let mut chunks = vec![];
	let terrain_scale = 20.;
	let terrain_offset = CHUNK_FLOOR as f32;

        // Yes this is slow, but it will be fine for now
        for i in -n_chunks..n_chunks + 1 {
            for j in -n_chunks..n_chunks + 1 {
                let mut chunk = Chunk::new([i as f32 * s, j as f32 * s]);

		// get the height from the perlin noise for each block
		for x in 0..8 {
		    for z in 0..8 {
			let height = terrain_offset
			    + terrain_scale * noise.at([i as f32 * s + x as f32,
							j as f32 * s + z as f32]);

			let cube_height = height.floor() as i32;

			for y in 0..cube_height - 1 {
			    chunk.add_cube(Vector3::new(i as f32 * s + x as f32,
							y as f32,
							j as f32 * s + z as f32),
					   DIRT, true);
			}
			chunk.add_cube(Vector3::new(i as f32 * s + x as f32,
						    cube_height as f32,
						    j as f32 * s + z as f32),
				       GRASS, true);
		    }
		}
		
                chunks.push(chunk);
            }
        }

        let mut world = Self { chunks };
        world.compute_visible_cubes();
        world
    }

    
    /// Loads a world from a file.
    pub fn from_file(name: &str) -> Option<Self> {
        match std::fs::read_to_string(name) {
            Ok(data) => Some(Self::from_json(data)),
            Err(err) => {
                println!("Could not read: {name} with error: {err}");
                None
            }
        }
    }

    /// Saves the current map to the given file
    pub fn save_to_file(&self, name: &str) {
        // Note: so far I am using `serde_json` but we will be able to change this in the future.
        //       There seems to be many options suited for us: https://serde.rs/#data-formats
        let serialized = self.to_json();
        match std::fs::write(name, serialized) {
            Ok(_) => println!("Map was saved at {name}"),
            Err(err) => {
                println!("Error while saving {name}: {err}")
            }
        }
    }

    /// Returns a list of cube attributes to be drawn on the screen.
    /// Each item on this list will result in a cube drawn in the screen.
    ///
    /// 'selected_cube': the currently selected cube, that will be rendered differently.
    pub fn get_cubes_to_draw(&self, selected_cube: Option<Vector3>) -> Vec<CubeAttr> {
        // I know that this function looks bad, but... Trust the optimizer
        // I have tried to optimize this shit using a custom class that does not re-allocate everything
        // but it does not improve anything ... So let's keep the simple solution of always calling `push`
        let mut positions: Vec<CubeAttr> = Vec::new();
        for chunk in &self.chunks {
            for layer in chunk.cubes() {
                for row in layer {
                    for cube in row {
                        if let Some(c) = cube {
                            // TODO is there no other fucking way to check whether this cube is the selected one ? 
                            //      let's think in term of performance...
                            if c.is_visible() {
                                let is_selected = selected_cube.is_some() && selected_cube.unwrap().to_cube_coordinates().equals(c.position());
                                positions.push(CubeAttr::new(c.model_matrix(), c.block_id(), is_selected));
                            }
                        }
                    }
                }
            }
        }
        positions
    }

    /// Returns true if there is a cube at this position
    pub fn is_position_free(&self, pos: &Vector3) -> bool {
        for chunk in &self.chunks {
            if chunk.is_in(pos) {
                if !chunk.is_position_free(pos) {
                    return false;
                }
            }
        }
        true
    }

    /// Returns true if the given position is free falling
    pub fn is_position_free_falling(&self, pos: &Vector3) -> bool {
        for chunk in &self.chunks {
            if chunk.is_in(pos) {
                if !chunk.is_position_free_falling(pos) {
                    return false;
                }
            }
        }
        true
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Destroy { at } => self.destroy_cube(at),
            Action::Add { at, block } => self.add_cube(at, block, true)
        }
    }
    
    fn cube_at_mut(&mut self, pos: Vector3) -> Option<&mut Cube> {
        for chunk in &mut self.chunks {
            if chunk.is_in(&pos) {
                return chunk.cube_at_mut(&pos)
            }
        }
        None
    }

    fn add_cube(&mut self, at: Vector3, block: Block, visible: bool) {
        for chunk in &mut self.chunks {
            if chunk.is_in(&at) {
                chunk.add_cube(at, block, visible)
            }
        }
        // TODO recompute the visiblity of the cube below ?
    }

    fn destroy_cube(&mut self, at: Vector3) {
        // Find the chunk where the cube is located
        let mut chunk_index = 0;
        for i in 0..self.chunks.len() {
            if self.chunks[i].is_in(&at) {
                chunk_index = i;
                break;
            }
        }
        
        // Mark all the neighbors cube as visible
        if let Some(cube) = self.chunks[chunk_index].cube_at(&at) {
            for pos in cube.neighbors_positions() {
                // Toggle this position
                if let Some(cube_to_toggle) = self.cube_at_mut(pos) {
                    cube_to_toggle.set_is_visible(true);
                }
            }
        }
        
        self.chunks[chunk_index].destroy_cube(at);
    }

    fn visible_cubes_count(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.visible_cube_count()).sum()
    }

    /// Goes through all the cubes in the world, and sets whether the cube is touching air.
    fn compute_visible_cubes(&mut self) {
        // Note to the reader:
        // This function works in two-step. First, the inside of each chunk is passed through.
        // It is easy because each chunk does not need information about the external world.
        // The second part is much harder: we look at the borders of each chunk. It is harder
        // because in rust, it is not possible to do something like this
        //
        // for chunk in &mut self.chunks {
        //     for pos in chunk.border() {
        //         if self.is_position_free(pos) {...}
        //     }
        // }
        //
        // It is not possible because `self` is borrowed as mutable, so it is impossible to
        // call any function with `self.ANY_FUNCTION(...)`
        // As a consequence of this limitation, I had to split the process in two extra steps:
        // (1) collect the indices of the cubes of the border to be marked as not visible and
        // (2) mark them as not visible.


        // 1. First pass inside each chunk

        for chunk in &mut self.chunks {
            chunk.compute_visible_cubes();
        }

        // 2. Handle the borders of each chunk

        // a. First, determine which cubes needs to be set as not visible.
        let mut indices_to_set_as_not_visible: Vec<Vec<CubeIndex>> = vec![Vec::new(); self.chunks.len()];
        for (i, chunk) in self.chunks.iter().enumerate() {
            let border = chunk.border();
            for index in border {
                if let Some(cube) = chunk.cube_at_index(index) {
                    let to_check = cube.neighbors_positions();
                    // If any of the neighbors is free, then the border is visible.
                    let is_visible = to_check.iter().any(|pos| self.is_position_free(&pos));
                    if !is_visible {
                        indices_to_set_as_not_visible[i].push(index);
                    }
                }
            }
        }

        // b. Go through these indices, but now apply the modification that needs to be applied
        for (i, chunk) in self.chunks.iter_mut().enumerate() {
            for index in &indices_to_set_as_not_visible[i] {
                if let Some(cube) = chunk.cube_at_index_mut(*index) {
                    cube.set_is_visible(false);
                }
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::block_kind::Block::GRASS;
    use crate::chunk::{Chunk, CHUNK_FLOOR, CHUNK_SIZE};
    use crate::vector::Vector3;
    use crate::world::World;

    #[test]
    fn test_chunk_collision_1() {
        let mut world = World::new();
        // Adding one chunk
        let s = CHUNK_SIZE as f32;
        world.chunks.push(Chunk::new_for_demo([-s, 0.], 0));
        world.chunks[0].print_all_cubes();

        // Assert some positions
        assert!(!world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 - 1.5, 4.0)));
        assert!(!world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 - 0.5, 4.0)));
        assert!(!world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 + 0.5, 4.0)));
        assert!(world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 + 1.5, 4.0)));
        assert!(world.is_position_free(&Vector3::new(-4.0, CHUNK_FLOOR as f32 + 1.5, 4.0)));
    }

    #[test]
    fn test_chunk_collision_2() {
        let mut world = World::new();
        // Adding one chunk
        world.chunks.push(Chunk::new_for_demo([0., 0.], 0));
        assert!(world.is_position_free(&Vector3::new(4.0, 10.2, 3.0)));
    }

    #[test]
    fn test_visible_cube_with_two_chunks_that_touch() {
        let mut world = World::new();
        let mut chunk1 = Chunk::new([0., 0.]);
        chunk1.fill_layer(0, GRASS);
        chunk1.fill_layer(1, GRASS);
        chunk1.fill_layer(2, GRASS);

        let mut chunk2 = Chunk::new([CHUNK_SIZE as f32, 0.]);
        chunk2.fill_layer(0, GRASS);
        chunk2.fill_layer(1, GRASS);
        chunk2.fill_layer(2, GRASS);

        world.chunks.push(chunk1);
        world.chunks.push(chunk2);

        let count1 = world.visible_cubes_count();
        world.compute_visible_cubes();
        let count2 = world.visible_cubes_count();

        assert!(count1 > count2);
        assert_eq!(count1, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE);
        assert_eq!(count2, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE - 1 * (CHUNK_SIZE - 2) * 2 * CHUNK_SIZE);
    }

    #[test]
    fn test_visible_cube_with_two_chunks_that_doesnt_touch() {
        let mut world = World::new();
        let mut chunk1 = Chunk::new([0., 0.]);
        chunk1.fill_layer(0, GRASS);
        chunk1.fill_layer(1, GRASS);
        chunk1.fill_layer(2, GRASS);

        let mut chunk2 = Chunk::new([3.*CHUNK_SIZE as f32, 0.]);
        chunk2.fill_layer(0, GRASS);
        chunk2.fill_layer(1, GRASS);
        chunk2.fill_layer(2, GRASS);

        world.chunks.push(chunk1);
        world.chunks.push(chunk2);

        let count1 = world.visible_cubes_count();
        world.compute_visible_cubes();
        let count2 = world.visible_cubes_count();

        assert!(count1 > count2);
        assert_eq!(count1, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE);

        // In this case, there is no border between the two cubes.
        assert_eq!(count2, 2 * 3 * CHUNK_SIZE * CHUNK_SIZE - 2 * (CHUNK_SIZE - 2) * (CHUNK_SIZE - 2));
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedWorld {
    chunk_corners: Vec<[f32;2]>,
    cubes_by_kind: HashMap<Block, Vec<[i32;4]>>
}

impl World {

    fn to_json(&self) -> String {
        
        // Provide all the chunks corner
        let chunk_corners: Vec<[f32;2]> = self.chunks.iter().map(|chunk| chunk.corner()).collect();
        
        // Provide all the cubes, sorted by kind
        let mut all_cubes = HashMap::new();
        for block_kind in Block::iter() {
            all_cubes.insert(block_kind, Vec::<([i32; 4])>::new());
        }

        for chunk in &self.chunks {
            for cube in chunk.flattened_iter() {
                if let Some(cube) = cube {
                    // we can trust that the block has a container.
                    let container = all_cubes.get_mut(cube.block()).unwrap();
                    container.push([
                        cube.position().x() as i32,
                        cube.position().y() as i32,
                        cube.position().z() as i32,
                        cube.is_visible() as i32
                    ]);
                }
            }
        }

        let world = SerializedWorld {
            chunk_corners,
            cubes_by_kind: all_cubes
        };
        serde_json::to_string(&world).unwrap()

    }

    fn from_json(data: String) -> Self {
        // If we end up with stack-overflows, we could not read the entire file but instead provide the reader.
        let serialized_world: SerializedWorld = serde_json::from_str(data.as_str()).unwrap();

        // First, build all the chunks
        let mut chunks = Vec::new();
        for corner in serialized_world.chunk_corners {
            chunks.push(Chunk::new(corner));
        }

        // Build the world
        let mut world = Self {
            chunks
        };

        // Fill all the chunks by building all the cubes
        for block_kind in Block::iter() {
            let cubes = serialized_world.cubes_by_kind.get(&block_kind).unwrap();
            for cube_data in cubes {
                let x = cube_data[0] as f32;
                let y = cube_data[1] as f32;
                let z = cube_data[2] as f32;
                let visible = cube_data[3] != 0;
                world.add_cube(Vector3::new(x,y,z), block_kind, visible);
            }
        }

        world
    }
}
