use crate::chunk::Chunk;
use crate::network::server_update::ServerUpdate::{LoadChunk, Response};
use std::str::from_utf8;

pub const RESPONSE_OK: u8 = 100;
pub const RESPONSE_ERROR: u8 = 101;

/// List of messages that are sent to the client from the server
#[derive(Clone, Debug)]
pub enum ServerUpdate {
    /// Ask the client to load a new chunk
    LoadChunk(Chunk),
    Response(u8),
}

impl ServerUpdate {
    fn to_u8(&self) -> u8 {
        match self {
            LoadChunk(_) => 0,
            Response(_) => 1
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Compute the data inside the message
        let mut data = match self {
            LoadChunk(chunk) => chunk.to_json().into_bytes(),
            Response(code) => vec![*code]
        };

        // First bytes contains the type
        let mut data_to_send = vec![self.to_u8()];

        // Second 4-bytes contain the length of the message
        let len = data.len() as u32;
        for n in len.to_le_bytes() {
            data_to_send.push(n);
        }

        // Finally, append all the bytes of the message
        data_to_send.append(&mut data);

        // Finally add an 'EOL' to the packet
        data_to_send.push(b'\n');

        data_to_send
    }

    pub fn from_bytes(bytes: &[u8], size: usize) -> Vec<Self> {
        let mut to_return = vec![];

        let mut start = 0;

        loop {
            // Read the header
            // - type of the enum
            // - length of the message being sent
            let length_bytes: [u8; 4] = bytes[start + 1..start + 5].try_into().unwrap();
            let len = u32::from_le_bytes(length_bytes) as usize;
            let code = bytes[start];

            // This line is interesting for debugging.
            // println!("start = {start}, len = {}, end = {}, size = {size}", len + 5, start + 5 + len);

            // Depending on the type of the enum, parse correctly the content
            match code {
                0 => {
                    let as_json = from_utf8(&bytes[start + 5..start + 5 + len]).unwrap();
                    let chunk = Chunk::from_json(as_json);
                    to_return.push(LoadChunk(chunk));
                }
                1 => {
                    to_return.push(Response(bytes[start + 5]))
                }
                _ => eprintln!("Cannot build server update from code {code}")
            }

            // Increase the header
            start += len + 5;
            // Increase the footer
            start += 1;
            // Safety
            if start + 5 >= size {
                break;
            }
        }

        to_return
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;
    use crate::network::server_update::ServerUpdate;
    use crate::network::server_update::ServerUpdate::{LoadChunk, Response};

    #[test]
    fn test_load_chunks_encoding_decoding() {
        let chunk = Chunk::new_for_demo([3., 5.], 5);
        let update = LoadChunk(chunk);
        let bytes = update.to_bytes();
        let parsed = ServerUpdate::from_bytes(bytes.as_slice(), bytes.len());

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
        let bytes = update.to_bytes();
        let parsed = ServerUpdate::from_bytes(bytes.as_slice(), bytes.len());

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

        let mut bytes1 = update_1.to_bytes();
        let mut bytes2 = update_2.to_bytes();
        let mut bytes3 = update_3.to_bytes();

        bytes1.append(&mut bytes2);
        bytes1.append(&mut bytes3);

        let parsed = ServerUpdate::from_bytes(bytes1.as_slice(), bytes1.len());
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
