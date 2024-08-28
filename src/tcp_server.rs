use crafty::game_server::GameServer;
use crafty::tcp_protocol::{MessageToServer, Response};
use crafty::world::World;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{io, thread};

const ONE_SEC: Duration = Duration::from_millis(1000);

fn handle_client(mut stream: TcpStream, game: Arc<Mutex<GameServer>>) {
    let mut data = [0 as u8; 50];
    let mut client_id = 0;

    // A while loop that continues to work for as long as the server lives.
    // This TCP stream is set to non-blocking, this is why there is a thread::sleep at the end of the
    // loop.
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                // The problem is that when we reach this place, there can be 'several' messages tied together.
                // So this means that we must be able to detect if there is more than 1 message in this 'queue' 
                let messages = MessageToServer::parse(&data, size);
                for message in messages {
                    let response = match message {
                        MessageToServer::Login => {
                            let id = game.lock().unwrap().login("player") as u8;
                            client_id = id as usize;
                            id
                        }
                        MessageToServer::OnNewPosition(new_pos) => {
                            game.lock().unwrap().on_new_position_update(client_id, new_pos);
                            Response::OK.to_u8()
                        }
                        MessageToServer::OnNewAction(_) => {
                            Response::ERROR.to_u8()
                        }
                    };

                    // Send the response to the client
                    stream.write(&[response]).unwrap();
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Read: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_nonblocking
            }
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
        
        // Check if the server has some updates to send to the client, and if so forward them !
        let updates = game.lock().unwrap().consume_updates(client_id);
        println!("Server has {} updates for client {}", updates.len(), client_id);
        
        for update in updates {
            
            
            
        }

        thread::sleep(Duration::from_millis(10));
    }
}

enum WorldInitializer {
    RANDOM,
    FLAT,
    DISK,
}

fn main() {
    
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

    // Start the Server
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    println!("Server is running: {}", listener.local_addr().unwrap());

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Set the stream also as non-blocking on the server-side.
                stream.set_nonblocking(true).expect("Cannot set non-blocking");

                // Create a new thread that will handle the connection with this client
                // Note that each client must be able to send messages back to the world
                let new_game = game.clone();
                thread::spawn(move || handle_client(stream, new_game));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // In the non-blocking mode, this branch is called when the server is not receiving any new connections
                // Read: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_nonblocking
                continue;
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    // close the socket server
    drop(listener);
}