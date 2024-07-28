
mod cube;
mod camera;
mod world_renderer;
mod world;

use crate::world_renderer::WorldRenderer;

fn main() {
    let mut renderer = WorldRenderer::new();
    renderer.run();
}