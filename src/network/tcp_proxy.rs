use crate::actions::Action;
use crate::network::message_to_server::MessageToServer;
use crate::network::server_update::ServerUpdate;
use crate::network::proxy::Proxy;
use crate::vector::Vector3;
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
    // We use 2 ** 15 so that it is possible to send big messages, such as a full chunk
    let mut data = [0u8; 2_usize.pow(17)];
    // let mut data = vec![];

    // First, send a logging request to the server
    // This is the first thing to do
    // We also keep track of the last message, to know how to parse the answer.
    let mut last_message_sent = None;

    // Unlike in the server, this loop is blocking.
    // This will need to be changed, obviously
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                for update in ServerUpdate::from_bytes(&data, size) {
                    proxy.lock().unwrap().push_server_update(update);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => println!("Failed to receive data: {}", e)
        }

        // Try to read if the WorldRenderer tried to communicate something to the server
        match updates_receiver.try_recv() {
            Ok(message) => {
                // Keep track of what was our last message sent
                last_message_sent = Some(message);
                // Send the message to the server
                stream.write(last_message_sent.as_ref().unwrap().to_bytes().as_slice()).unwrap();
            }
            Err(_) => {}
        }
    }
}

/// A connection to the server using a TCP stream over the network
pub struct TcpProxy {
    client_id: usize,
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
                client_id: 0,
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

    fn set_client_id(&mut self, id: usize) {
        println!("Client was assigned ID {id} by server.");
        self.client_id = id;
    }
}

impl Proxy for TcpProxy {
    fn login(&mut self) {
        self.updates_transmitter.send(MessageToServer::Login).unwrap();
    }

    fn send_position_update(&mut self, position: Vector3) {
        // TODO maybe we could call this only when the player is moving
        match self.updates_transmitter.send(MessageToServer::OnNewPosition(position)) {
            Ok(_) => {}
            Err(err) => println!("Error while sending: {err}")
        }
    }

    fn on_new_action(&mut self, action: Action) {
        todo!()
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
}