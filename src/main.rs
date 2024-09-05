
mod cube;
mod aabb;
mod camera;
mod world_renderer;
mod world;
mod world_generation;
mod chunk;
mod vector;
mod graphics;
mod actions;
mod fps;
mod block_kind;

use crate::camera::Camera;
use crate::world::World;
use crate::world_renderer::WorldRenderer;

fn main() {
    // pick your prefered world gen
    let mut world = World::create_new_flat_world(10);
    // let mut world = World::create_new_random_world(10);
    world.save_to_file("map.json");
    let cam = Camera::new();
    let mut renderer = WorldRenderer::new(world, cam);
    renderer.run();
}


