use crafty::game_server::GameServer;
use crafty::network::tcp_server::TcpServer;
use crafty::world::World;
use std::sync::{Arc, Mutex};
use crafty::args::Args;

#[allow(dead_code)]
enum WorldInitializer {
    RANDOM,
    FLAT,
    DISK,
}

fn main() {
    let args = Args::from_args();
    let url = args.url();
    
    // Create the initial world
    let init = WorldInitializer::FLAT;
    println!("[Server] Creating a world ...");
    let world = match init {
        WorldInitializer::RANDOM => World::create_new_random_world(10),
        WorldInitializer::FLAT => World::create_new_flat_world(10),
        WorldInitializer::DISK => World::from_file("map.json").unwrap()
    };
    println!("                          ... Finished !");

    // Create the game model of the server.
    // It holds the 'full' world
    // It is put inside an ARC to be shared across each thread, and inside a Mute to have interior mutability.
    let game = Arc::new(Mutex::new(GameServer::new(world)));

    TcpServer::start(&url, game)
}