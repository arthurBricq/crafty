#[derive(PartialEq, Debug)]
pub struct EntityAttack{
    pub attacked: u8,
}

impl EntityAttack {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.attacked]
    }

    pub fn from_bytes(bytes_to_parse: &[u8]) -> Self {
        Self { attacked: bytes_to_parse[0] }
    }
}