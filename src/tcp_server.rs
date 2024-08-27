use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; 
    
    // A while loop that continues to work for as long as the server lives.
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            // stream.write(&data[0..size]).unwrap();
            
            println!("Received: {:?}", &data[0..size]);
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                
                // Create a new thread that will handle the connection with this client
                // Note that each client must be able to send messages back to the world
                thread::spawn(move || handle_client(stream) );
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