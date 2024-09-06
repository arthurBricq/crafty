use crate::actions::Action;
use crate::network::message_to_server::MessageToServer::{Login, OnNewAction, OnNewPosition};
use crate::network::tcp_message_encoding::{TcpDeserialize, TcpSerialize};
use crate::primitives::position::Position;
use std::str::from_utf8;

/// List of message that can be exchanged between to the server from the client
#[derive(Debug, PartialEq)]
pub enum MessageToServer {
    /// Ask the server to log in a new player
    /// The response provides the client with the player ID
    Login,
    OnNewPosition(Position),
    OnNewAction(Action),
}

impl TcpSerialize for MessageToServer {
    fn to_u8(&self) -> u8 {
        match self {
            Login => 0,
            OnNewPosition(_) => 1,
            OnNewAction(_) => 2
        }
    }

    fn to_bytes_representation(&self) -> Vec<u8> {
        match self {
            Login => vec![],
            OnNewPosition(pos) => pos.to_bytes(),
            OnNewAction(action) => action.to_bytes(),
        }
    }
}

impl TcpDeserialize for MessageToServer {
    fn parse_bytes_representation(code: u8, bytes_to_parse: &[u8]) -> Self {
        match code {
            0 => Login,
            1 => OnNewPosition(Position::from_bytes(bytes_to_parse)),
            2 => OnNewAction(Action::from_str(from_utf8(bytes_to_parse).unwrap())),
            _ => panic!("Cannot build message to server from code {code}")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::message_to_server::MessageToServer;
    use crate::network::message_to_server::MessageToServer::{Login, OnNewPosition};
    use crate::network::tcp_message_encoding::{from_tcp_repr, to_tcp_repr, ParseContext};
    use crate::primitives::position::Position;
    use crate::primitives::vector::Vector3;

    fn test_integrity(m: MessageToServer) {
        let bytes = to_tcp_repr(&m);
        let mut context = ParseContext::new();
        let parsed = from_tcp_repr(bytes.as_slice(), &mut context);
        assert_eq!(m, parsed[0]);
    }

    #[test]
    fn test_message_integrity() {
        test_integrity(Login);
        test_integrity(OnNewPosition(Position::new_vec(1.0, 1.0, 1.0)));
        test_integrity(OnNewPosition(Position::new_vec(-1.0, 2.0, 100.012)));
    }

    fn test_multiple_messages(messages: &[MessageToServer]) {
        let bytes  = messages
            .iter()
            .map(|m| to_tcp_repr(m))
            .collect::<Vec<Vec<u8>>>()
            .concat();
        let mut context = ParseContext::new();
        let parsed = from_tcp_repr(bytes.as_slice(), &mut context);
        assert_eq!(messages.len(), parsed.len());
        for (i, m) in messages.iter().enumerate() {
            assert_eq!(*m, parsed[i]);
        }
    }

    #[test]
    fn test_multiple_message_integrity() {
        let p1 = Vector3::new(1., 2., 3.);
        let p2 = Vector3::new(5., 6., 7.);
        test_multiple_messages(&[OnNewPosition(Position::from_pos(p1.clone())), OnNewPosition(Position::from_pos(p2))]);
        test_multiple_messages(&[Login, OnNewPosition(Position::from_pos(p1))]);
    }
}

