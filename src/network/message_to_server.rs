use crate::actions::Action;
use crate::network::message_to_server::MessageToServer::{Login, OnNewAction, OnNewPosition};
use crate::vector::Vector3;
use std::str::from_utf8;

const HEADER_SIZE: usize = 5;

/// List of message that can be exchanged between to the server from the client
#[derive(Debug, PartialEq)]
pub enum MessageToServer {
    /// Ask the server to log in a new player
    /// The response provides the client with the player ID
    Login,
    OnNewPosition(Vector3),
    OnNewAction(Action),
}

impl MessageToServer {
    pub fn parse(data: &[u8], size: usize) -> Vec<Self> {
        // Potentially, there have been several messages squashed together
        // Therefore we make sure to split with '~' which is our message delimiter.
        // Note: this function is probably not well optimized. We might reconsider its implementation in the future.
        let full_message = from_utf8(&data[0..size]).unwrap();
        let mut messages = vec![];
        for message in full_message.split('~') {
            if message.len() < HEADER_SIZE { continue; }
            match &message[0..HEADER_SIZE] {
                "login" => messages.push(Login),
                "posit" => {
                    // Parse the end of the message into a new position
                    let remaining = &message[HEADER_SIZE..];
                    let mut pos = Vector3::empty();
                    for (i, part) in remaining.split(',').enumerate() {
                        pos[i] = part.parse::<f32>().unwrap();
                    }
                    messages.push(OnNewPosition(pos));
                }
                // TODO change this stupid header format. Use an integer instead.
                "actio" => todo!(),
                _ => {}
            };
        }

        messages
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Login => b"login".to_vec(),
            OnNewPosition(pos) => format!("posit{},{},{}~", pos.x(), pos.y(), pos.z()).into_bytes(),
            OnNewAction(_) => panic!("not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::message_to_server::MessageToServer;
    use crate::vector::Vector3;

    fn test_integrity(m: MessageToServer) {
        let bytes = m.to_bytes();
        let parsed = MessageToServer::parse(bytes.as_slice(), bytes.len());
        assert_eq!(m, parsed[0]);
    }

    #[test]
    fn test_message_integrity() {
        test_integrity(MessageToServer::Login);
        test_integrity(MessageToServer::OnNewPosition(Vector3::new(1.0, 1.0, 1.0)));
        test_integrity(MessageToServer::OnNewPosition(Vector3::new(-1.0, 2.0, 100.012)));
    }
}


