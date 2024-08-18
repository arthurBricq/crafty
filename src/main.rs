
mod cube;
mod camera;
mod world_renderer;
mod world;
mod chunk;
mod vector;
mod gravity;
mod graphics;
mod actions;
mod fps;

use crate::camera::Camera;
use crate::world::World;
use crate::world_renderer::WorldRenderer;

fn main() {
    let mut world = World::new();
    world.fill_for_demo();
    let cam = Camera::new();
    let mut renderer = WorldRenderer::new(world, cam);
    renderer.run();
}


