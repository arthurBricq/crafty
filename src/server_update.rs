use crate::chunk::Chunk;

#[derive(Clone, Debug)]
/// List of messages that are sent to the client from the server
pub enum ServerUpdate {
    /// Ask the client to load a new chunk
    LoadChunk(Chunk),
    None
}

impl ServerUpdate {

    pub fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }
}
