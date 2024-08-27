use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc;
use std::{io, thread};
use std::time::{Duration, Instant};

const ONE_SEC: Duration = Duration::from_millis(1000);

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    let mut t = Instant::now();

    // A while loop that continues to work for as long as the server lives.
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                println!("Received: {:?}", &data[0..size]);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Read: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_nonblocking
                if t.elapsed() > ONE_SEC {
                    // TODO every second, send a message to the client
                    t = Instant::now();
                    let message = b"action::add_cube";
                    stream.write(message).unwrap();
                }
            }
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
        
        thread::sleep(Duration::from_millis(10));
    }
    
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");
    println!("Server is running: {}", listener.local_addr().unwrap());

    // let (tx, rx) = mpsc::channel();

    // Accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Set the stream also as non-blocking on the server-side.
                stream.set_nonblocking(true).expect("Cannot set non-blocking");

                // Create a new thread that will handle the connection with this client
                // Note that each client must be able to send messages back to the world
                thread::spawn(move || handle_client(stream) );
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