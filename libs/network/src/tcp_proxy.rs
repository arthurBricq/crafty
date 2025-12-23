use crate::message_to_server::MessageToServer;
use crate::proxy::{ClientToServer, Proxy, ServerToClient};
use crate::tcp_message_encoding::{from_tcp_repr, to_tcp_repr, ParseContext};
use async_channel::{Receiver as AsyncReceiver, Sender as AsyncSender};
use model::game::actions::Action;
use model::game::attack::EntityAttack;
use model::server::server_update::ServerUpdate;
use primitives::position::Position;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::{io, thread};
use tracing::{error, info};

/// Function that handles the thread that
/// - sends messages to server
/// - receives updates from server
fn handle_stream_with_server(
    mut stream: TcpStream,
    proxy: Arc<Mutex<TcpProxy>>,
    updates_receiver: Receiver<MessageToServer>,
    updates_sender: AsyncSender<Vec<ServerUpdate>>,
) {
    // Buffer of data for the stream
    let mut data = [0u8; 2_usize.pow(17)];

    let mut context = ParseContext::new();

    loop {
        // Continuously read the bytes received by the server
        match stream.read(&mut data) {
            Ok(size) => {
                match from_tcp_repr(&data[0..size], &mut context) {
                    Ok(updates) => {
                        // Send updates to async channel
                        if updates_sender.try_send(updates).is_err() {
                            error!("Failed to send update to async channel");
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse TCP message: {}", e);
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => error!("Failed to receive data: {}", e),
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
    updates_receiver: AsyncReceiver<Vec<ServerUpdate>>,
}

impl TcpProxy {
    /// Returns an Arc to a proxy connected to a remote server
    /// A new thread is instantiated that contains the logic of communicating with the remote server
    pub fn new(server_address: &str) -> Arc<Mutex<Self>> {
        let (tx, rx) = mpsc::channel();
        let (async_tx, async_rx) = async_channel::unbounded();

        let proxy = Arc::new(Mutex::new(Self {
            updates_transmitter: tx,
            updates_receiver: async_rx,
        }));

        // Start a stream on a new thread
        match TcpStream::connect(server_address) {
            Ok(stream) => {
                info!("Successfully connected to server");
                stream
                    .set_nonblocking(true)
                    .expect("Cannot set non-blocking");
                let new_proxy = proxy.clone();
                thread::spawn(move || handle_stream_with_server(stream, new_proxy, rx, async_tx));
            }
            Err(e) => {
                error!("Failed to connect: {}", e);
            }
        }

        proxy
    }
}

impl ClientToServer for TcpProxy {
    async fn login(&mut self, name: String) {
        match self.updates_transmitter.send(MessageToServer::Login(name)) {
            Ok(_) => {}
            Err(err) => panic!("Error while logging in: {err}"),
        }
    }

    async fn send_position_update(&mut self, position: Position) {
        match self
            .updates_transmitter
            .send(MessageToServer::OnNewPosition(position))
        {
            Ok(_) => {}
            Err(err) => error!("Error while sending: {err}"),
        }
    }

    async fn on_new_action(&mut self, action: Action) {
        match self
            .updates_transmitter
            .send(MessageToServer::OnNewAction(action))
        {
            Ok(_) => {}
            Err(err) => error!("Error while sending: {err}"),
        }
    }

    async fn on_new_attack(&mut self, attack: EntityAttack) {
        match self
            .updates_transmitter
            .send(MessageToServer::Attack(attack))
        {
            Ok(_) => {}
            Err(err) => error!("Error while sending: {err}"),
        }
    }

    async fn request_to_spawn(&mut self, position: Position) {
        match self
            .updates_transmitter
            .send(MessageToServer::SpawnRequest(position))
        {
            Ok(_) => {}
            Err(err) => error!("Error while sending: {err}"),
        }
    }
}

impl ServerToClient for TcpProxy {
    async fn next_updates(&mut self) -> Option<Vec<ServerUpdate>> {
        self.updates_receiver.recv().await.ok()
    }
}

impl Proxy for TcpProxy {}
