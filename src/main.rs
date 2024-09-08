use std::env;
use std::env::Args;
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

impl WorldInitializer {
    fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();
        println!("args = {args:?}");
        if args.contains(&"--random".to_string()) {
            Self::RANDOM
        }
        else if args.contains(&"--flat".to_string()) {
            Self::FLAT
        }
        else if args.contains(&"--disk".to_string()) {
            Self::DISK
        }
        else {
            Self::RANDOM
        }
    }
}

fn main() {
    // Create the initial world
    let init = WorldInitializer::from_args();
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
