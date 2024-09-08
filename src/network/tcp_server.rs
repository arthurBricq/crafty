use std::{io, thread};
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::server::game_server::GameServer;
use crate::network::message_to_server::MessageToServer;

use crate::network::tcp_message_encoding::{from_tcp_repr, to_tcp_repr, ParseContext};

pub struct TcpServer {

}

impl TcpServer {
    pub fn start(address: &str, game: Arc<Mutex<GameServer>>) {
        // Start the Server
        let listener = TcpListener::bind(address).unwrap();
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
}

fn handle_client(mut stream: TcpStream, game: Arc<Mutex<GameServer>>) {
    let mut data = [0_u8; 2_usize.pow(10)];
    let mut client_id = None;
    let mut context = ParseContext::new();

    // A while loop that continues to work for as long as the server lives.
    // This TCP stream is set to non-blocking, this is why there is a thread::sleep at the end of the
    // loop.
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                // Read the messages sent by the client
                match from_tcp_repr::<MessageToServer>(&data[0..size], &mut context) {
                    Ok(messages) => {
                        // For each message, create a response and send it to the client.
                        for message in messages {
                            match message {
                                MessageToServer::Login(name) => {
                                    let id = game.lock().unwrap().login(name) as u8;
                                    // The thread memorizes
                                    client_id = Some(id as usize);
                                }
                                MessageToServer::OnNewPosition(new_pos) => {
                                    game.lock().unwrap().on_new_position_update(client_id.unwrap(), new_pos);
                                }
                                MessageToServer::OnNewAction(action) => {
                                    println!("Client {client_id:?} has submitted an action: {action:?}");
                                    game.lock().unwrap().on_new_action(client_id.unwrap(), action);
                                }
                                MessageToServer::Attack(attack) => {
                                    
                                    
                                }
                            };
                        }
                    }
                    Err(_) => {
                        println!("Error while communicating with client: {client_id:?}");
                        if let Some(id) = client_id {
                            game.lock().unwrap().logout(id)
                        }
                        println!("Trying to safely shutdown...");
                        match stream.shutdown(Shutdown::Write) {
                            Ok(_) =>          println!("   ... Shutdown successfull"),
                            Err(err) => println!("   ... Error while closing socket: {err}")
                        }
                        return;
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Read: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_nonblocking
            }
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
        }

        // Check if the server has some updates to send to the client, and if so forward them !
        if let Some(id) = client_id {
            let updates = game.lock().unwrap().consume_updates(id);
            if updates.len() > 0 {
                for update in &updates {
                    let msg = to_tcp_repr(update);
                    match stream.write_all(msg.as_slice()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error while sending message to client {client_id:?}: {e}");
                            return;
                        }
                    }
                    stream.flush().unwrap();

                    if update.is_heavy() {
                        thread::sleep(Duration::from_millis(20));
                    }
                }
            }
        }

    }
}
