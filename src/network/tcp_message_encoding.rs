use crate::network::server_update::ServerUpdate;

/// A trait that an enum or a struct implement to be shared over the network.
/// This trait can be used by `to_tcp_repr` to encode a message on our custom protocol.
pub trait TcpSerialize {
    /// Returns a code that describes which variant of the type is being encoded.
    fn to_u8(&self) -> u8;
    /// Returns the vector of bytes representing the object.
    fn to_bytes_representation(&self) -> Vec<u8>;
}

/// A trait that an enum or a struct implement to be shared over the network.
/// This trait can be used by `to_tcp_repr` to decode a message on our custom protocol.
pub trait TcpDeserialize {
    fn parse_bytes_representation(code: u8, bytes_to_parse: &[u8]) -> Self;
}

/// Given an object that can be serialized to our TCP protocol,
/// returns the bytes message to be sent over the network
pub fn to_tcp_repr<T: TcpSerialize>(object: &T) -> Vec<u8>{
    let mut data = object.to_bytes_representation();

    // First bytes contains the type
    let mut data_to_send = vec![object.to_u8()];

    // Second 4-bytes contain the length of the message
    let len = data.len() as u32;
    for n in len.to_le_bytes() {
        data_to_send.push(n);
    }

    // Finally, append all the bytes of the message
    data_to_send.append(&mut data);
    data_to_send
}

pub fn from_tcp_repr<T: TcpDeserialize>(bytes: &[u8], size: usize) -> Vec<T> {
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
        println!("start = {start}, len = {}, end = {}, size = {size}", len + 5, start + 5 + len);
        if start + 5 + len > size {
            println!("ERROR. A chunk has been clipped.");
            break;
        }

        // Depending on the type of the enum, parse correctly the content
        let bytes_to_parse = &bytes[start+5..start+5+len];
        let parsed = T::parse_bytes_representation(code, bytes_to_parse);
        to_return.push(parsed);

        // Increase the counter
        start += len + 5;
        if start + 5 >= size {
            break;
        }
    }

    to_return
}