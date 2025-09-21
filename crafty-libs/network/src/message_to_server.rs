use std::str::from_utf8;
use model::game::actions::Action;
use model::game::attack::EntityAttack;
use primitives::position::Position;
use crate::message_to_server::MessageToServer::{Attack, Login, OnNewAction, OnNewPosition, SpawnRequest};
use crate::tcp_message_encoding::{TcpDeserialize, TcpSerialize};

/// List of message that can be exchanged between to the server from the client
#[derive(Debug, PartialEq)]
pub enum MessageToServer {
    /// Ask the server to log in a new player with a given identifer
    Login(String),
    OnNewPosition(Position),
    OnNewAction(Action),
    Attack(EntityAttack),
    SpawnRequest(Position),
}

impl TcpSerialize for MessageToServer {
    fn to_u8(&self) -> u8 {
        match self {
            Login(_) => 0,
            OnNewPosition(_) => 1,
            OnNewAction(_) => 2,
            Attack(_) => 3,
            SpawnRequest(_) => 4,
        }
    }

    fn to_bytes_representation(&self) -> Vec<u8> {
        match self {
            Login(name) => name.clone().into_bytes(),
            OnNewPosition(pos) | SpawnRequest(pos) => pos.to_bytes(),
            OnNewAction(action) => action.to_bytes(),
            Attack(attack) => attack.to_bytes(),
        }
    }
}

impl TcpDeserialize for MessageToServer {
    fn parse_bytes_representation(code: u8, bytes_to_parse: &[u8]) -> Self {
        match code {
            0 => Login(from_utf8(bytes_to_parse).unwrap().to_string()),
            1 => OnNewPosition(Position::from_bytes(bytes_to_parse)),
            2 => OnNewAction(Action::from_str(from_utf8(bytes_to_parse).unwrap())),
            3 => Attack(EntityAttack::from_bytes(bytes_to_parse)),
            4 => SpawnRequest(Position::from_bytes(bytes_to_parse)),
            _ => panic!("Cannot build message to server from code {code}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use primitives::position::Position;
    use primitives::vector::Vector3;
    use crate::message_to_server::MessageToServer;
    use crate::message_to_server::MessageToServer::{Login, OnNewPosition};
    use crate::tcp_message_encoding::{from_tcp_repr, to_tcp_repr, ParseContext};

    fn test_integrity(m: MessageToServer) {
        let bytes = to_tcp_repr(&m);
        let mut context = ParseContext::new();
        let parsed = from_tcp_repr(bytes.as_slice(), &mut context).unwrap();
        assert_eq!(m, parsed[0]);
    }

    #[test]
    fn test_message_integrity() {
        test_integrity(Login("arthur".to_string()));
        test_integrity(OnNewPosition(Position::new_vec(1.0, 1.0, 1.0)));
        test_integrity(OnNewPosition(Position::new_vec(-1.0, 2.0, 100.012)));
    }

    fn test_multiple_messages(messages: &[MessageToServer]) {
        let bytes = messages
            .iter()
            .map(|m| to_tcp_repr(m))
            .collect::<Vec<Vec<u8>>>()
            .concat();
        let mut context = ParseContext::new();
        let parsed = from_tcp_repr(bytes.as_slice(), &mut context).unwrap();
        assert_eq!(messages.len(), parsed.len());
        for (i, m) in messages.iter().enumerate() {
            assert_eq!(*m, parsed[i]);
        }
    }

    #[test]
    fn test_multiple_message_integrity() {
        let p1 = Vector3::new(1., 2., 3.);
        let p2 = Vector3::new(5., 6., 7.);
        test_multiple_messages(&[
            OnNewPosition(Position::from_pos(p1.clone())),
            OnNewPosition(Position::from_pos(p2)),
        ]);
        test_multiple_messages(&[
            Login("hey".to_string()),
            OnNewPosition(Position::from_pos(p1)),
        ]);
    }
}
