use crate::aabb::AABB;
use crate::actions::Action;
use crate::block_kind::Block;
use crate::chunk::{Chunk, CHUNK_SIZE};
use crate::collidable::{Collidable, CollisionData};
use crate::cube::Cube;
use crate::cubes_to_draw::CubesToDraw;
use crate::graphics::cube::CubeInstance;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;
use crate::world_serializer::{get_serialize_container, serialize_one_chunk, SerializedWorld};
use glium::glutin::surface::WindowSurface;
use glium::{Display, VertexBuffer};
use strum::IntoEnumIterator;

pub struct World {
    /// The list of the chunks currently being displayed
    chunks: Vec<Chunk>,
    cubes_to_draw: Option<CubesToDraw>,
}

impl World {
    pub fn empty() -> Self {
        let chunks = Vec::new();
        Self {
            chunks,
            cubes_to_draw: None,
        }
    }

    pub fn new(chunks: Vec<Chunk>) -> Self {
        let mut w = Self {
            chunks,
            cubes_to_draw: None,
        };

        w.compute_visible_cubes();
        w
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

    pub fn cubes_near_player(&self, pos: Vector3) -> impl Iterator<Item = &Option<Cube>> {
        self.chunks
            .iter()
            .filter(move |chunk| chunk.is_near_player(&pos))
            .flat_map(|chunk| chunk.cubes_iter())
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        if self.cubes_to_draw.is_some() {
            self.cubes_to_draw.as_mut().unwrap().add_chunk(&chunk);
        }
        self.chunks.push(chunk);
    }

    pub fn get_chunk(&self, corner: (i32, i32)) -> Option<Chunk> {
        for chunk in &self.chunks {
            let tmp = chunk.corner();
            if tmp[0] == corner.0 as f32 && tmp[1] == corner.1 as f32 {
                return Some(chunk.clone());
            }
        }
        None
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
    pub fn set_cubes_to_draw(&mut self) {
        // I know that this function looks bad, but... Trust the optimizer
        // I have tried to optimize this shit using a custom class that does not re-allocate everything
        // but it does not improve anything ... So let's keep the simple solution of always calling `push`
        let mut positions: Vec<CubeInstance> = Vec::new();
        for chunk in &self.chunks {
            for layer in chunk.cubes() {
                for row in layer {
                    for cube in row {
                        if let Some(c) = cube {
                            if c.is_visible() {
                                positions.push(CubeInstance::new(c));
                            }
                        }
                    }
                }
            }
        }
        if self.cubes_to_draw.is_none() {
            self.cubes_to_draw = Some(CubesToDraw::new());
        }
        self.cubes_to_draw
            .as_mut()
            .unwrap()
            .set_cube_to_draw(positions);
    }

    pub fn cube_to_draw(&self) -> &[CubeInstance] {
        self.cubes_to_draw.as_ref().unwrap().cubes_to_draw()
    }

    /// Returns the OpenGL buffer with cubes to be drawn
    /// If you want to have one cube drawn as 'selected', pass it in the argument `selected`
    pub fn get_cubes_buffer(
        &mut self,
        display: &Display<WindowSurface>,
        selected: Option<Cube>,
    ) -> VertexBuffer<CubeInstance> {
        self.cubes_to_draw
            .as_mut()
            .unwrap()
            .get_cubes_buffer(display, selected)
    }

    pub fn number_cubes_rendered(&self) -> usize {
        self.cubes_to_draw.as_ref().unwrap().number_cubes_rendered()
    }

    /// Returns the block at the given position
    pub fn block_at(&self, pos: &Vector3) -> Option<Block> {
        for chunk in &self.chunks {
            if chunk.is_in(pos) {
                if let Some(cube) = chunk.cube_at(pos) {
                    return Some(cube.block().clone());
                }
            }
        }

        None
    }

    /// Returns true if there is a cube at this position
    pub fn is_position_free_or_transparent(&self, pos: &Vector3) -> bool {
        for chunk in &self.chunks {
            if chunk.is_in(pos) {
                if !chunk.is_position_free_or_transparent(pos) {
                    return false;
                }
            }
        }
        true
    }

    pub fn apply_action(&mut self, action: &Action) {
        match action {
            Action::Destroy { at } => {
                let revealed_cubes = self.destroy_cube(at.clone());
                if self.cubes_to_draw.is_some() {
                    // Remove the cube from the rendered cube
                    self.cubes_to_draw.as_mut().unwrap().remove_cube(&at);
                    // When a cube is supress some of its neighbors might become visible and have to be added to the cube_to_draw
                    for cube in revealed_cubes {
                        self.cubes_to_draw.as_mut().unwrap().add_cube(&cube)
                    }
                }
            }
            Action::Add { at, block } => {
                let (cubes_to_destroy, cube) = self.add_cube(at.clone(), block.clone());
                if self.cubes_to_draw.is_some() {
                    // Add the cube from the rendered cube
                    self.cubes_to_draw.as_mut().unwrap().add_cube(&cube);
                    // When a cube is added some of its neighbors might become invisible and have to be remove to the cube_to_draw
                    for position in cubes_to_destroy {
                        self.cubes_to_draw.as_mut().unwrap().remove_cube(&position)
                    }
                }
            }
        }
    }

    fn cube_at_mut(&mut self, pos: Vector3) -> Option<&mut Cube> {
        for chunk in &mut self.chunks {
            if chunk.is_in(&pos) {
                return chunk.cube_at_mut(&pos);
            }
        }
        None
    }

    pub fn cube_at(&self, pos: Vector3) -> Option<&Cube> {
        for chunk in &self.chunks {
            if chunk.is_in(&pos) {
                return chunk.cube_at(&pos);
            }
        }
        None
    }

    /// Adds a cube and then recomputes the visibility of the affected cubes (neighbors)
    /// Return the cube that need to be rendered
    /// and the position of the rendered cube that need to be destroyed
    fn add_cube(&mut self, at: Vector3, block: Block) -> (Vec<Vector3>, Cube) {
        // For all the neighbors positions, increase their internal counter
        let mut count = 0;
        let mut to_hide = Vec::new();
        if !block.is_transparent() {
            for pos in Cube::neighbors_positions(at) {
                // Toggle this position
                if let Some(cube_to_toggle) = self.cube_at_mut(pos) {
                    // This cube now has a new neighbor
                    cube_to_toggle.add_neighhor();
                    if !cube_to_toggle.is_visible() {
                        to_hide.push(cube_to_toggle.position().clone())
                    }
                    count += 1;
                }
            }
        }
        self.add_cube_unsafe(at, block, count);
        (to_hide, self.cube_at_mut(at).unwrap().clone())
    }

    /// Adds a cube without recomputing the visibility
    fn add_cube_unsafe(&mut self, at: Vector3, block: Block, neighbors: u8) {
        for chunk in &mut self.chunks {
            if chunk.is_in(&at) {
                chunk.add_cube(at, block, neighbors);
            }
        }
    }

    /// Destroy a cube and return the neighboring cubes that need to be rendered
    fn destroy_cube(&mut self, at: Vector3) -> Vec<Cube> {
        // Find the chunk where the cube is located
        let mut chunk_index = 0;
        for i in 0..self.chunks.len() {
            if self.chunks[i].is_in(&at) {
                chunk_index = i;
                break;
            }
        }

        let mut cubes_to_reveal = Vec::new();
        // Mark all the neighbors cube as visible
        if let Some(_cube) = self.chunks[chunk_index].cube_at(&at) {
            for pos in Cube::neighbors_positions(at) {
                if let Some(cube_to_toggle) = self.cube_at_mut(pos) {
                    // If the cube was not visible before, add it
                    if !cube_to_toggle.is_visible() {
                        cubes_to_reveal.push(cube_to_toggle.clone());
                    }
                    cube_to_toggle.remove_neighbor();
                }
            }
        }

        // Shouldn't this be inside the if let ?
        self.chunks[chunk_index].destroy_cube(at);
        cubes_to_reveal
    }

    #[cfg(test)]
    fn visible_cubes_count(&self) -> usize {
        self.chunks
            .iter()
            .map(|chunk| chunk.visible_cube_count())
            .sum()
    }

    /// Goes through all the cubes in the world, and sets whether the cube is touching air.
    fn compute_visible_cubes(&mut self) {
        // 1. First pass inside each chunk
        for chunk in &mut self.chunks {
            chunk.compute_visible_cubes();
        }

        // 2. Handle the borders of each chunk
        for i in 0..self.chunks.len() {
            let border = self.chunks[i].border();
            for index in border {
                // Count the number of neighbors of this cube
                let mut count = if let Some(cube_at_border) = self.chunks[i].cube_at_index(index) {
                    let neighbors = Cube::neighbors_positions(cube_at_border.position().clone());
                    let count = neighbors
                        .iter()
                        .filter(|pos| !self.is_position_free_or_transparent(&pos))
                        .count();
                    count as u8
                } else {
                    0
                };

                // If it is the bottommost layer, increase
                if index.0 == 0 {
                    count += 1;
                }

                // Set it
                // You need to do this separatly than the previous block.
                if let Some(cube_at_border) = self.chunks[i].cube_at_index_mut(index) {
                    cube_at_border.set_n_neighbors(count);
                }
            }
        }
    }
}

impl World {
    fn to_json(&self) -> String {
        // Provide all the chunks corner
        let chunk_corners: Vec<[f32; 2]> = self.chunks.iter().map(|chunk| chunk.corner()).collect();

        // Provide all the cubes, sorted by kind
        let mut all_cubes = get_serialize_container();
        for chunk in &self.chunks {
            serialize_one_chunk(&mut all_cubes, chunk);
        }

        let world = SerializedWorld {
            chunk_corners,
            cubes_by_kind: all_cubes,
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
            chunks,
            cubes_to_draw: None,
        };

        // Fill all the chunks by building all the cubes
        for block_kind in Block::iter() {
            let cubes = serialized_world.cubes_by_kind.get(&block_kind).unwrap();
            for cube_data in cubes {
                let x = cube_data[0] as f32;
                let y = cube_data[1] as f32;
                let z = cube_data[2] as f32;
                let neighbors = cube_data[3] as u8;
                world.add_cube_unsafe(Vector3::new(x, y, z), block_kind, neighbors);
            }
        }

        world
    }
}

impl Collidable for World {
    fn collides(&self, aabb: &AABB) -> bool {
        for chunk in &self.chunks {
            if chunk.collides(aabb) {
                return true;
            }
        }

        false
    }

    // now returns collision time; f32::MAX if no collision
    fn collision_time(
        &self,
        position: &Position,
        aabb: &AABB,
        target: &AABB,
        velocity: &Vector3,
    ) -> Option<CollisionData> {
        // find with which chunks it is colliding
        let mut acc_time = f32::MAX;
        let mut acc_normal = Vector3::empty();

        // TODO be smarter
        for chunk in &self.chunks {
            if chunk.is_near_player(&position.pos()) {
                if let Some(CollisionData { time, normal }) =
                    chunk.collision_time(position, aabb, target, velocity)
                {
                    if time < acc_time {
                        acc_time = time;
                        acc_normal = normal;
                    }
                }
            }
        }

        if acc_time > 1e10 {
            None
        } else {
            Some(CollisionData {
                time: acc_time,
                normal: acc_normal,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::Action;
    use crate::block_kind::Block;
    use crate::block_kind::Block::GRASS;
    use crate::chunk::{Chunk, CHUNK_FLOOR, CHUNK_SIZE};
    use crate::collidable::Collidable;
    use crate::entity::humanoid::humanoid_aabb;
    use crate::primitives::position::Position;
    use crate::primitives::vector::Vector3;
    use crate::world::World;
    use crate::world_generation::world_generator::WorldGenerator;
    use std::time::Instant;

    #[test]
    fn test_chunk_collision_1() {
        let mut world = World::empty();
        // Adding one chunk
        let s = CHUNK_SIZE as f32;
        world.chunks.push(Chunk::new_for_demo([-s, 0.], 0));
        world.chunks[0].print_all_cubes();

        // Assert some positions
        assert!(!world.is_position_free_or_transparent(&Vector3::new(
            -4.0,
            CHUNK_FLOOR as f32 - 1.5,
            4.0
        )));
        assert!(!world.is_position_free_or_transparent(&Vector3::new(
            -4.0,
            CHUNK_FLOOR as f32 - 0.5,
            4.0
        )));
        assert!(!world.is_position_free_or_transparent(&Vector3::new(
            -4.0,
            CHUNK_FLOOR as f32 + 0.5,
            4.0
        )));
        assert!(world.is_position_free_or_transparent(&Vector3::new(
            -4.0,
            CHUNK_FLOOR as f32 + 1.5,
            4.0
        )));
        assert!(world.is_position_free_or_transparent(&Vector3::new(
            -4.0,
            CHUNK_FLOOR as f32 + 1.5,
            4.0
        )));
    }

    #[test]
    fn test_chunk_collision_2() {
        let mut world = World::empty();
        // Adding one chunk
        world.chunks.push(Chunk::new_for_demo([0., 0.], 0));
        assert!(world.is_position_free_or_transparent(&Vector3::new(4.0, 10.2, 3.0)));
    }

    #[test]
    fn test_visible_cube_with_two_chunks_that_touch() {
        let mut world = World::empty();
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
    }

    #[test]
    fn test_visible_cube_with_two_chunks_that_doesnt_touch() {
        let mut world = World::empty();
        let mut chunk1 = Chunk::new([0., 0.]);
        chunk1.fill_layer(0, GRASS);
        chunk1.fill_layer(1, GRASS);
        chunk1.fill_layer(2, GRASS);

        let mut chunk2 = Chunk::new([3. * CHUNK_SIZE as f32, 0.]);
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
    }

    #[test]
    fn test_visibility_after_deleting_cubes() {
        let mut world = World::empty();
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        chunk.fill_layer(1, GRASS);
        chunk.fill_layer(2, GRASS);
        world.chunks.push(chunk);
        world.compute_visible_cubes();

        let top = Vector3::new(4., 2., 4.);
        let middle = Vector3::new(4., 1., 4.);
        let bottom = Vector3::new(4., 0., 4.);

        // Initially, the cube in the middle is not supposed to be visible
        // Note that the bottommost layer is not showed
        assert_eq!(world.chunks[0].cube_at(&top).unwrap().is_visible(), true);
        assert_eq!(
            world.chunks[0].cube_at(&middle).unwrap().is_visible(),
            false
        );
        assert_eq!(
            world.chunks[0].cube_at(&bottom).unwrap().is_visible(),
            false
        );

        // Now we delete the top cube
        world.apply_action(&Action::Destroy { at: top });

        // Assert the cube in the middle is now visible
        assert_eq!(world.chunks[0].cube_at(&middle).unwrap().is_visible(), true);

        // But so far, the sides of `bottom` should not be visible yet
        let one_side = middle + Vector3::unit_x();
        let another_side = middle + Vector3::unit_z();
        assert_eq!(
            world.chunks[0].cube_at(&one_side).unwrap().is_visible(),
            false
        );
        assert_eq!(
            world.chunks[0].cube_at(&another_side).unwrap().is_visible(),
            false
        );

        // But we if delete the middle block, the sides get in contact with air, so they are supposed to be visible.
        world.apply_action(&Action::Destroy { at: middle });
        assert_eq!(
            world.chunks[0].cube_at(&one_side).unwrap().is_visible(),
            true
        );
        assert_eq!(
            world.chunks[0].cube_at(&another_side).unwrap().is_visible(),
            true
        );
    }

    #[test]
    fn test_visibility_after_creating_and_deleting_cubes() {
        let mut world = World::empty();
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        chunk.fill_layer(1, GRASS);
        chunk.fill_layer(2, GRASS);
        world.chunks.push(chunk);
        world.compute_visible_cubes();

        let above = Vector3::new(4., 3., 4.);
        let top = Vector3::new(4., 2., 4.);

        // First, we add a cube on top of the world
        world.apply_action(&Action::Add {
            at: above,
            block: Block::COBBELSTONE,
        });

        // Assert the visibility: the block 'top' should not be rendered anymore
        assert_eq!(world.chunks[0].cube_at(&above).unwrap().is_visible(), true);
        assert_eq!(world.chunks[0].cube_at(&top).unwrap().is_visible(), false);
    }

    #[test]
    fn test_visibility_of_bottommost_layer() {
        let mut world = World::empty();
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        chunk.fill_layer(1, GRASS);
        chunk.fill_layer(2, GRASS);
        world.chunks.push(chunk);
        world.compute_visible_cubes();
        let bottom = Vector3::new(4., 0., 4.);
        assert_eq!(
            world.chunks[0].cube_at(&bottom).unwrap().is_visible(),
            false
        );
    }

    #[test]
    fn test_world_persistence() {
        let world = WorldGenerator::create_new_random_world(2);
        let serialized = world.to_json();
        let reconstructed = World::from_json(serialized);
        assert_eq!(world.chunks, reconstructed.chunks);
    }

    #[test]
    fn test_cube_iter() {
        let mut world = World::empty();
        let mut chunk = Chunk::new([0., 0.]);
        chunk.fill_layer(0, GRASS);
        world.chunks.push(chunk);

        let count = world
            .cubes_near_player(Vector3::empty())
            .filter(|c| c.is_some())
            .count();
        assert_eq!(count, CHUNK_SIZE * CHUNK_SIZE)
    }

    #[test]
    fn benchmark_collision() {
        let world = World::from_file("benchmark_map.json").unwrap();

        let t0 = Instant::now();
        for i in 0..100 {
            let pos = Position::new(Vector3::new(0., 100., 2.), 0., 0.);
            let from = humanoid_aabb(&pos);
            let velocity = Vector3::unit_x();
            let dt = 0.01;
            let target = humanoid_aabb(&(&pos + velocity * dt));

            let collision = world.collision_time(&pos, &from, &target, &velocity);
        }
        println!("Elasped = {:?}", t0.elapsed());
    }
}
