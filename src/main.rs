use crafty::game_server::GameServer;
use crafty::network::single_player_proxy::SinglePlayerProxy;
use crafty::world::World;
use crafty::world_renderer::WorldRenderer;
use crafty::{camera::Camera, world_generation::world_generator::WorldGenerator};
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
enum WorldInitializer {
    RANDOM, 
    FLAT,
    DISK,
}

fn main() {
    // Create the initial world
    let init = WorldInitializer::RANDOM;
    let world = match init {
        WorldInitializer::RANDOM => WorldGenerator::create_new_random_world(10),
        WorldInitializer::FLAT => WorldGenerator::create_new_flat_world(10),
        WorldInitializer::DISK => World::from_file("map.json").unwrap(),
    };

    // The server holds the 'full' world
    let server = GameServer::new(world);

    // The proxy currently holds the server,
    let proxy = SinglePlayerProxy::new(server);

    // The client is initialized with an empty world, as it will be the responsibility of the server
    // to provide it with the chunks.
    // Currently, the client 'owns' the proxy, this is really the part that sucks for now.
    let mut renderer =
        WorldRenderer::new(Arc::new(Mutex::new(proxy)), World::empty(), Camera::new());
    renderer.login();
    renderer.run();
    /*
     */
}
