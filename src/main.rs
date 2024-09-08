use crafty::server::game_server::GameServer;
use crafty::network::single_player_proxy::SinglePlayerProxy;
use crafty::world::World;
use crafty::world_renderer::WorldRenderer;
use crafty::{player::Player, world_generation::world_generator::WorldGenerator};
use std::sync::{Arc, Mutex};
use crafty::network::proxy::Proxy;

#[allow(dead_code)]
#[derive(Debug)]
enum WorldInitializer {
    RANDOM,
    FLAT,
    DISK,
}

fn main() {
    // Create the initial world
    let init = WorldInitializer::DISK;
    println!("Loading world using : {:?}", init);
    let world = match init {
        WorldInitializer::RANDOM => WorldGenerator::create_new_random_world(5),
        WorldInitializer::FLAT => WorldGenerator::create_new_flat_world(10),
        WorldInitializer::DISK => World::from_file("map.json").unwrap_or(WorldGenerator::create_new_random_world(10)),
    };

    // The server holds the 'full' world
    let server = GameServer::new(world);

    // The proxy currently holds the server,
    let mut proxy = SinglePlayerProxy::new(server);
    proxy.login("local_client".to_string());

    // The client is initialized with an empty world, as it will be the responsibility of the server
    // to provide it with the chunks.
    // Currently, the client 'owns' the proxy, this is really the part that sucks for now.
    let mut renderer =
        WorldRenderer::new(Arc::new(Mutex::new(proxy)), World::empty(), Player::new());
    renderer.run();
    /*
     */
}
