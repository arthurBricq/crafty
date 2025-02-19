use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum TcpError {
    LengthError,
}

impl Display for TcpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while communicating over TCP: {:?}", self)
    }
}

impl std::error::Error for TcpError {}

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
pub fn to_tcp_repr<T: TcpSerialize>(object: &T) -> Vec<u8> {
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

pub struct ParseContext {
    bytes: Vec<u8>,
    code: u8,
    len: usize,
}

impl ParseContext {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            len: 0,
            code: 0,
        }
    }

    fn flush(&mut self) {
        self.bytes = vec![];
        self.len = 0;
    }

    fn store(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data)
    }

    fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    fn set_message_len(&mut self, len: usize) {
        self.len = len;
    }

    fn remaining_length_to_read(&self) -> usize {
        self.len - self.bytes.len()
    }

    fn set_code(&mut self, code: u8) {
        self.code = code;
    }
}

pub fn from_tcp_repr<T: TcpDeserialize>(
    bytes: &[u8],
    context: &mut ParseContext,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut to_return = vec![];
    let mut start = 0;

    loop {
        if bytes.len() < start + 5 {
            break;
        }

        if context.is_empty() {
            // It means it's a new message to be read.
            // Read the header
            // - type of the enum
            // - length of the message being sent
            let length_bytes: [u8; 4] = bytes
                .get(start + 1..start + 5)
                .ok_or(TcpError::LengthError)?
                .try_into()?;
            let len = u32::from_le_bytes(length_bytes) as usize;
            let code = bytes[start];
            context.set_code(code);
            context.set_message_len(len);
            start += 5;
        }

        let remaining = context.remaining_length_to_read();

        // This line is interesting for debugging.
        // println!("[TCP] start = {start}, len = {}, size = {}, remaining = {}, code = {}", context.len, bytes.len(), remaining, context.code);

        if start + remaining > bytes.len() {
            // This means that the message sent was too small to be sent over 1 byte
            // So we have to wait for the next message
            context.store(&bytes[start..]);

            // Wait for next message
            break;
        } else {
            context.store(&bytes[start..start + remaining]);
        }

        // Once we arrive here, we know that we can parse 1 message.

        // Depending on the type of the enum, parse correctly the content
        let parsed = T::parse_bytes_representation(context.code, &context.bytes);
        to_return.push(parsed);
        context.flush();

        // Increase the counter, in the case that there are several messaages to be parsed
        // in the current packet.
        start += remaining;
        if start >= bytes.len() {
            break;
        }
    }

    Ok(to_return)
}
