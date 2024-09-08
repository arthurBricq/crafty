use crate::actions::Action;
use crate::chunk::Chunk;
use crate::network::server_update::ServerUpdate::{LoadChunk, LoggedIn, RegisterEntity, SendAction, UpdatePosition};
use crate::network::tcp_message_encoding::{TcpDeserialize, TcpSerialize};

use std::str::from_utf8;
use crate::primitives::position::Position;

pub const RESPONSE_OK: u8 = 100;
pub const RESPONSE_ERROR: u8 = 101;

/// List of messages that are sent to the client from the server
#[derive(Clone, Debug)]
pub enum ServerUpdate {
    /// Ask the client to load a new chunk
    LoadChunk(Chunk),
    /// The server forwards to the client his client id
    LoggedIn(u8, Position),
    /// The server forwards to the client an action to be executed
    SendAction(Action),
    /// Tell the client that a new player is part of the game
    RegisterEntity(u8, Position),
    /// Update the position of an existing entity
    UpdatePosition(u8, Position)
}

impl ServerUpdate {
    /// Returns true if this update is big enough to require a special treatment when sent over the
    /// network.
    pub fn is_heavy(&self) -> bool {
        matches!(self, LoadChunk(_))
    }
}

impl TcpSerialize for ServerUpdate {
    fn to_u8(&self) -> u8 {
        match self {
            LoadChunk(_) => 0,
            LoggedIn(_, _) => 1,
            SendAction(_) => 2,
            RegisterEntity(_, _) => 3,
            UpdatePosition(_, _) => 4
        }
    }

    fn to_bytes_representation(&self) -> Vec<u8> {
        // Compute the data inside the message
        match self {
            LoadChunk(chunk) => chunk.to_json().into_bytes(),
            SendAction(action) => action.to_bytes(),
            LoggedIn(id, pos) | RegisterEntity(id, pos) | UpdatePosition(id, pos)=> {
                let mut bytes = vec![*id];
                bytes.extend_from_slice(&pos.to_bytes());
                bytes
            }
        }
    }
}

impl TcpDeserialize for ServerUpdate {
    fn parse_bytes_representation(code: u8, bytes_to_parse: &[u8]) -> ServerUpdate {
        match code {
            0 => {
                let as_json = from_utf8(bytes_to_parse).unwrap();
                let chunk = Chunk::from_json(as_json);
                match chunk {
                    Ok(chunk) => LoadChunk(chunk),
                    Err(err) => panic!("Error while parsing a chunk: {err}")
                }
            }
            1 => {
                LoggedIn(bytes_to_parse[0], Position::from_bytes(&bytes_to_parse[1..]))
            }
            2 => {
                let as_json = from_utf8(bytes_to_parse).unwrap();
                let action = Action::from_str(as_json);
                SendAction(action)
            }
            3 => {
                RegisterEntity(bytes_to_parse[0], Position::from_bytes(&bytes_to_parse[1..]))
            }
            4 => {
                UpdatePosition(bytes_to_parse[0], Position::from_bytes(&bytes_to_parse[1..]))
            }
            _ => panic!("Cannot build server update from code {code}")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::network::server_update::ServerUpdate;
    use crate::network::server_update::ServerUpdate::{LoadChunk, LoggedIn, RegisterEntity};
    use crate::network::tcp_message_encoding::{from_tcp_repr, to_tcp_repr, ParseContext};
    use crate::primitives::position::Position;
    use crate::primitives::vector::Vector3;

    #[test]
    fn test_load_chunks_encoding_decoding() {
        let chunk = Chunk::new_for_demo([3., 5.], 5);
        let update = LoadChunk(chunk);
        let bytes = to_tcp_repr(&update);
        let mut context = ParseContext::new();
        let parsed = from_tcp_repr::<ServerUpdate>(bytes.as_slice(), &mut context).unwrap();

        // Assert that the two chunks are the same !
        match (&update, &parsed[0]) {
            (LoadChunk(a), LoadChunk(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }
    }

    #[test]
    fn test_response_encoding_decoding() {
        let update = LoggedIn(113, Position::empty());
        let bytes = to_tcp_repr(&update);
        let mut context = ParseContext::new();
        let parsed = from_tcp_repr::<ServerUpdate>(bytes.as_slice(), &mut context).unwrap();

        // Assert that the two chunks are the same !
        match (&update, &parsed[0]) {
            (LoggedIn(a, _), LoggedIn(b, _)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }
    }

    #[test]
    fn test_parse_multiple_updates_at_one() {
        let chunk1 = Chunk::new_for_demo([3., 5.], 5);
        let chunk2 = Chunk::new_for_demo([31., -52.], 10);

        let update_1 = LoadChunk(chunk1);
        let update_2 = LoadChunk(chunk2);
        let update_3 = LoggedIn(113, Position::empty());
        let update_4 = RegisterEntity(113, Position::from_pos(Vector3::new(-3., 2., 34.532)));

        let mut bytes1 = to_tcp_repr(&update_1);
        let mut bytes2 = to_tcp_repr(&update_2);
        let mut bytes3 = to_tcp_repr(&update_3);
        let mut bytes4 = to_tcp_repr(&update_4);

        bytes1.append(&mut bytes2);
        bytes1.append(&mut bytes3);
        bytes1.append(&mut bytes4);

        let mut context = ParseContext::new();
        let parsed = from_tcp_repr::<ServerUpdate>(bytes1.as_slice(), &mut context).unwrap();
        assert_eq!(4, parsed.len());

        match (&update_1, &parsed[0]) {
            (LoadChunk(a), LoadChunk(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }

        match (&update_2, &parsed[1]) {
            (LoadChunk(a), LoadChunk(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }

        match (&update_3, &parsed[2]) {
            (LoggedIn(a, _), LoggedIn(b, _)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }

        match (&update_4, &parsed[3]) {
            (RegisterEntity(id1, pos1), RegisterEntity(id2, pos2)) => {
                assert_eq!(id1, id2);
                assert_eq!(pos1, pos2);
            },
            (_, _) => assert!(false)
        }
    }

    #[test]
    fn test_one_message_sent_over_mutliple_packet() {
        let chunk1 = Chunk::new_for_demo([3., 5.], 5);
        let update_1 = LoadChunk(chunk1);
        let bytes1 = to_tcp_repr(&update_1);

        let packet1 = &bytes1[0..500];
        let packet2 = &bytes1[500..1500];
        let packet3 = &bytes1[1500..];

        let mut context = ParseContext::new();

        let parsed: Vec<ServerUpdate> = from_tcp_repr(packet1, &mut context).unwrap();
        assert_eq!(0, parsed.len());
        
        let parsed: Vec<ServerUpdate> = from_tcp_repr(packet2, &mut context).unwrap();
        assert_eq!(0, parsed.len());
        
        let parsed: Vec<ServerUpdate> = from_tcp_repr(packet3, &mut context).unwrap();
        assert_eq!(1, parsed.len())
    }
}
