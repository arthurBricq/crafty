use crafty::args::{Args, WorldInitializer};
use crafty::network::tcp_server::TcpServer;
use crafty::server::game_server::{handle_entity_thread, GameServer};
use crafty::world::World;
use crafty::world_generation::world_generator::WorldGenerator;
use std::sync::{Arc, Mutex};


fn main() {
    let args = Args::from_args();

    // Create the initial world
    println!("[Server] Creating a world ...");
    let world = match args.init {
        WorldInitializer::RANDOM => WorldGenerator::create_new_random_world(10),
        WorldInitializer::FLAT => WorldGenerator::create_new_flat_world(10),
        WorldInitializer::DISK => World::from_file("map.json").unwrap(),
    };
    println!("                          ... Finished !");

    // Create the game model of the server.
    // It holds the 'full' world
    // It is put inside an ARC to be shared across each thread, and inside a Mute to have interior mutability.
    let game = Arc::new(Mutex::new(GameServer::new(world)));
    
    // Spawn the entity thead
    let ref1 = game.clone();
    std::thread::spawn(move || handle_entity_thread(ref1));
    
    // Starts the TCP server
    TcpServer::start(&args.url(), game)
}
