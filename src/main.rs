
mod cube;
mod camera;
mod world_renderer;
mod world;
mod chunk;

use crate::world_renderer::WorldRenderer;

fn main() {
    let mut renderer = WorldRenderer::new();
    renderer.run();
}