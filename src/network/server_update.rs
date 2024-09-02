use crate::chunk::Chunk;
use crate::network::server_update::ServerUpdate::{LoadChunk, Response, SendAction};
use std::str::from_utf8;
use crate::actions::Action;
use crate::network::tcp_message_encoding::{TcpDeserialize, TcpSerialize};

pub const RESPONSE_OK: u8 = 100;
pub const RESPONSE_ERROR: u8 = 101;

/// List of messages that are sent to the client from the server
#[derive(Clone, Debug)]
pub enum ServerUpdate {
    /// Ask the client to load a new chunk
    LoadChunk(Chunk),
    Response(u8),
    SendAction(Action)
}

impl TcpSerialize for ServerUpdate {
    fn to_u8(&self) -> u8 {
        match self {
            LoadChunk(_) => 0,
            Response(_) => 1,
            SendAction(_) => 2
        }
    }

    fn to_bytes_representation(&self) -> Vec<u8> {
        // Compute the data inside the message
        match self {
            LoadChunk(chunk) => chunk.to_json().into_bytes(),
            Response(code) => vec![*code],
            SendAction(action) => action.to_bytes()
        }
    }
}

impl TcpDeserialize for ServerUpdate {
    fn parse_bytes_representation(code: u8, bytes_to_parse: &[u8]) -> ServerUpdate {
        let parsed = match code {
            0 => {
                let as_json = from_utf8(bytes_to_parse).unwrap();
                let chunk = Chunk::from_json(as_json);
                LoadChunk(chunk)
            }
            1 => {
                Response(bytes_to_parse[0])
            }
            2 => {
                let as_json = from_utf8(bytes_to_parse).unwrap();
                let action = Action::from_str(as_json);
                SendAction(action)
            }
            _ => panic!("Cannot build server update from code {code}")
        };
        parsed
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::network::server_update::ServerUpdate;
    use crate::network::server_update::ServerUpdate::{LoadChunk, Response};
    use crate::network::tcp_message_encoding::{from_tcp_repr, to_tcp_repr};

    #[test]
    fn test_load_chunks_encoding_decoding() {
        let chunk = Chunk::new_for_demo([3., 5.], 5);
        let update = LoadChunk(chunk);
        let bytes = to_tcp_repr(&update);
        let parsed = from_tcp_repr::<ServerUpdate>(bytes.as_slice(), bytes.len());

        // Assert that the two chunks are the same !
        match (&update, &parsed[0]) {
            (LoadChunk(a), LoadChunk(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }
    }

    #[test]
    fn test_response_encoding_decoding() {
        let chunk = Chunk::new_for_demo([3., 5.], 5);
        let update = Response(113);
        let bytes = to_tcp_repr(&update);
        let parsed = from_tcp_repr::<ServerUpdate>(bytes.as_slice(), bytes.len());

        // Assert that the two chunks are the same !
        match (&update, &parsed[0]) {
            (Response(a), Response(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }
    }

    #[test]
    fn test_parse_multiple_updates_at_one() {
        let chunk1 = Chunk::new_for_demo([3., 5.], 5);
        let chunk2 = Chunk::new_for_demo([31., -52.], 10);

        let update_1 = LoadChunk(chunk1);
        let update_2 = LoadChunk(chunk2);
        let update_3 = Response(113);

        let mut bytes1 = to_tcp_repr(&update_1);
        let mut bytes2 = to_tcp_repr(&update_2);
        let mut bytes3 = to_tcp_repr(&update_3);

        bytes1.append(&mut bytes2);
        bytes1.append(&mut bytes3);

        let parsed = from_tcp_repr::<ServerUpdate>(bytes1.as_slice(), bytes1.len());
        assert_eq!(3, parsed.len());

        match (&update_1, &parsed[0]) {
            (LoadChunk(a), LoadChunk(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }

        match (&update_2, &parsed[1]) {
            (LoadChunk(a), LoadChunk(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }

        match (&update_3, &parsed[2]) {
            (Response(a), Response(b)) => assert_eq!(a, b),
            (_, _) => assert!(false)
        }
    }
}
