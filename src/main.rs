
mod cube;
mod camera;
mod world_renderer;
mod world;
mod world_generation;
mod chunk;
mod vector;
mod gravity;
mod graphics;
mod actions;
mod fps;
mod block_kind;
mod items_bar;
mod player_items;
mod proxy;
mod game_server;
mod world_dispatcher;

use crate::camera::Camera;
use crate::proxy::SinglePlayerProxy;
use crate::game_server::GameServer;
use crate::world::World;
use crate::world_renderer::WorldRenderer;

enum WorldInitializer {
    RANDOM, FLAT, DISK
}

fn main() {
    // Create the initial world
    let init = WorldInitializer::RANDOM;
    let world = match init  {
        WorldInitializer::RANDOM => World::create_new_random_world(10),
        WorldInitializer::FLAT => World::create_new_flat_world(10),
        WorldInitializer::DISK => World::from_file("map.json").unwrap()
    };

    // The server holds the 'full' world
    let server = GameServer::new(world);

    // The proxy currently holds the server,
    let proxy = SinglePlayerProxy::new(server);

    // The client is initialized with an empty world, as it will be the responsibility of the server
    // to provide it with the chunks.
    // Currently, the client 'owns' the proxy, this is really the part that sucks for now.
    let mut renderer = WorldRenderer::new(proxy, World::empty(), Camera::new());
    renderer.login();
    renderer.run();
    /*
     */
}


