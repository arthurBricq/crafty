use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn handle_stream_with_server(mut stream: TcpStream) {
    let msg = b"Hello!";
    stream.write(msg).unwrap();
    let mut data = [0 as u8; 516];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                let as_string = from_utf8(&data[0..size]).unwrap();
                println!("Received: {:?}", as_string);
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}

pub fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(stream) => {
            println!("Successfully connected to server in port 3333");
            handle_stream_with_server(stream);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

