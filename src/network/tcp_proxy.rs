use crate::actions::Action;
use crate::network::message_to_server::MessageToServer;
use crate::network::proxy::Proxy;
use crate::network::server_update::ServerUpdate;
use crate::network::tcp_message_encoding::{from_tcp_repr, to_tcp_repr, ParseContext};
use crate::primitives::position::Position;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::{io, thread};

/// Function that handles the thread that
/// - sends messages to server
/// - receives updates from server
fn handle_stream_with_server(mut stream: TcpStream, proxy: Arc<Mutex<TcpProxy>>, updates_receiver: Receiver<MessageToServer>) {
    // Buffer of data for the stream
    let mut data = [0u8; 2_usize.pow(17)];

    let mut context = ParseContext::new();

    loop {

        // Continuously read the bytes received by the server
        match stream.read(&mut data) {
            Ok(size) => {
                for update in from_tcp_repr(&data[0..size], &mut context).unwrap() {
                    proxy.lock().unwrap().push_server_update(update);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => println!("Failed to receive data: {}", e)
        }

        // Try to read if the WorldRenderer tried to communicate something to the server
        match updates_receiver.try_recv() {
            Ok(message) => {
                // Send the message to the server
                stream.write(to_tcp_repr(&message).as_slice()).unwrap();
            }
            Err(_) => {}
        }
    }
}

/// A connection to the server using a TCP stream over the network
pub struct TcpProxy {
    updates_transmitter: Sender<MessageToServer>,
    pending_updates: VecDeque<ServerUpdate>,
}

impl TcpProxy {
    /// Returns an Arc to a proxy connected to a remote server
    /// A new thread is instantiated that contains the logic of communicating with the remote server
    pub fn new(server_address: &str) -> Arc<Mutex<Self>> {
        let (tx, rx) = mpsc::channel();

        let proxy = Arc::new(Mutex::new(
            Self {
                updates_transmitter: tx,
                pending_updates: VecDeque::new(),
            }
        ));

        // Start a stream on a new thread
        match TcpStream::connect(server_address) {
            Ok(stream) => {
                println!("Successfully connected to server");
                stream.set_nonblocking(true).expect("Cannot set non-blocking");
                let new_proxy = proxy.clone();
                thread::spawn(move || handle_stream_with_server(stream, new_proxy, rx));
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }

        proxy
    }

    /// Adds a server update to be read by the client
    pub fn push_server_update(&mut self, update: ServerUpdate) {
        self.pending_updates.push_back(update);
    }
}

impl Proxy for TcpProxy {
    fn login(&mut self, name: String) {
        match self.updates_transmitter.send(MessageToServer::Login(name)) {
            Ok(_) => {}
            Err(err) => panic!("Error while logging in: {err}")
        }
    }

    fn send_position_update(&mut self, position: Position) {
        match self.updates_transmitter.send(MessageToServer::OnNewPosition(position)) {
            Ok(_) => {}
            Err(err) => println!("Error while sending: {err}")
        }
    }

    fn on_new_action(&mut self, action: Action) {
        match self.updates_transmitter.send(MessageToServer::OnNewAction(action)) {
            Ok(_) => {}
            Err(err) => println!("Error while sending: {err}")
        }
    }

    fn consume_server_updates(&mut self) -> Vec<ServerUpdate> {
        // TODO change the API to get something that complies more with the circular buffer
        //      for instance returning an iterator that consumes the front of the queue ?
        //      or just providing an interface like `pop(&mut self) -> Option<ServerUpdate>
        //      this seems like a better idea...
        // Read the updates sent by the server
        let mut tmp = vec![];
        while let Some(update) = self.pending_updates.pop_front() {
            tmp.push(update);
        }
        tmp
    }

    fn loading_delay(&self) -> u64 {
        3000
    }
}