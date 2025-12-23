use graphics::player::world_renderer::WorldRenderer;
use model::args::WorldInitializer;
use model::game::player::Player;
use model::server::game_server::{handle_entity_thread, GameServer};
use model::world::generation::world_generator::WorldGenerator;
use model::world::world::World;
use network::proxy::Proxy;
use network::single_player_proxy::SinglePlayerProxy;
use std::sync::{Arc, Mutex};

fn main() {
    // Create the initial world
    let init = WorldInitializer::from_args();

    println!("Loading world using : {:?}", init);
    println!("[Server] Creating a world ...");
    let world = match init {
        WorldInitializer::RANDOM => WorldGenerator::create_new_random_world(5),
        WorldInitializer::FLAT => WorldGenerator::create_new_flat_world(10),
        WorldInitializer::DISK => {
            World::from_file("map.json").unwrap_or(WorldGenerator::create_new_random_world(10))
        }
    };
    println!("                          ... Finished !");

    // The server holds the 'full' world
    let server = Arc::new(Mutex::new(GameServer::new(world)));

    // Spawn the entity thead
    let ref1 = server.clone();
    std::thread::spawn(move || handle_entity_thread(ref1));

    // The proxy currently holds the server,
    let mut proxy = SinglePlayerProxy::new(server);
    proxy.login("local_client".to_string());

    // The client is initialized with an empty world, as it will be the responsibility of the server
    // to provide it with the chunks.
    // Currently, the client 'owns' the proxy, this is really the part that sucks for now.
    let mut renderer =
        WorldRenderer::new(Arc::new(Mutex::new(proxy)), World::empty(), Player::new());

    // Run with `glium`
    // renderer.run::<graphics_glium::runtime::GliumRenderer>();

    // Run the `wgpu`
    renderer.run::<graphics_wgpu::runtime::WgpuRenderer>();
}
