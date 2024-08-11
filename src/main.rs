
mod cube;
mod camera;
mod world_renderer;
mod world;
mod chunk;

use crate::camera::Camera;
use crate::world::World;
use crate::world_renderer::WorldRenderer;

fn main() {
    let world = World::new();
    let mut cam = Camera::new(&world);
    let mut renderer = WorldRenderer::new(&world, &mut cam);
    renderer.run();
}